Rust-Deduper
===

This is a simple file duplicate finder written in Rust. This will compare files based on their Md5 hashes.

Usage
---

	$ cargo run [directory]

This will scan in `[directory]` trying to find duplicate files. A log will be written to a file called `duplicates`, which can be piped into another command in order to delete the duplicate files.
