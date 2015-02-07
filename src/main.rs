#![feature(io,path,core)]

extern crate crypto;

use std::old_io::{File, fs};
use std::old_io::fs::PathExtensions;
use std::collections::HashSet;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hashes = HashSet::new();
    let folder = fs::walk_dir(&Path::new("."));
    match folder {
        Ok(results) => {
            for file_path in results {
                if file_path.is_file() {
                    let hash = hash_file(&file_path);
                    if hashes.contains(&hash) {
                        println!("{}", file_path.display());
                    } else {
                        hashes.insert(hash);
                    }
                }
            }
        },
        Err(e) => println!("{}", e),
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
