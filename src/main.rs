extern crate crypto;

use std::old_io::{File, fs, FileType};
use std::old_io::fs::PathExtensions;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let folder = fs::walk_dir(&Path::new("."));
    match folder {
        Ok(results) => {
            for file_path in results {
                if file_path.is_file() {
                    hash_file(&file_path);
                }
            }
        },
        Err(e) => println!("{}", e),
    }
}

fn hash_file(p: &Path) {
    let mut md5 = Md5::new();
    println!("{}", p.display());
    let contents = File::open(p).read_to_end();
    match contents {
        Ok(good) => {
            md5.input(&good);
            println!("{}", md5.result_str());
        },
        Err(i) => println!("{}", i),
    }
}
