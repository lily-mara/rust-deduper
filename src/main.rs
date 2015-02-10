#![feature(io,path,core,os)]

extern crate crypto;
extern crate getopts;

use std::os;
use std::old_io::{File, fs};
use std::old_io::fs::PathExtensions;
use std::collections::HashSet;
use crypto::md5::Md5;
use crypto::digest::Digest;
use getopts::Options;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "directory", "set folder to scan (required)", "DIRECTORY");
    opts.optflag("r", "remove", "automatically delete the duplicate files");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let directory = match matches.opt_str("d") {
        Some(d) => d,
        None => {
            print_usage(&program, opts);
            return;
        },
    };

    let folder = fs::walk_dir(&Path::new(directory));

    let mut hashes = Box::new(HashSet::new());
    let mut duplicates = Box::new(HashSet::new());

    match folder {
        Ok(results) => {
            for file_path in results {
                if file_path.is_file() {
                    let hash = hash_file(&file_path);
                    if hashes.contains(&hash) {
                        println!("Duplicate! {}", file_path.display());
                        duplicates.insert(file_path);
                    } else {
                        hashes.insert(hash);
                    }
                }
            }
        },
        Err(e) => println!("{}", e),
    }

    println!("{} duplicates found, {} files scanned", duplicates.len(), duplicates.len() + hashes.len() - 1);

    let mut out_file = File::create(&Path::new("duplicates")).unwrap();
    let delete =  matches.opt_present("r");

    for dup in duplicates.iter() {
        if delete {
            fs::unlink(&dup);
        }
        out_file.write_str(format!("{}\n", dup.display()).as_slice());
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(brief.as_slice()));
}

fn hash_file(p: &Path) -> String {
    let mut md5 = Md5::new();
    let contents = File::open(p).read_to_end();
    match contents {
        Ok(good) => {
            md5.input(&good);
            let result = md5.result_str();
            return result;
        },
        Err(i) => {
            panic!(format!("{}", i));
        }
    }
}
