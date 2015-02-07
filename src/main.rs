#![feature(io,path,core,os)]

extern crate crypto;

use std::os;
use std::old_io::{File, fs};
use std::old_io::fs::PathExtensions;
use std::collections::HashSet;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let args = os::args();
    let folder = fs::walk_dir(&Path::new(&args[1]));

    let mut hashes = HashSet::new();
    let mut duplicates = HashSet::new();

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
    for dup in duplicates {
        out_file.write_str(format!("{}\n", dup.display()).as_slice());
    }
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
