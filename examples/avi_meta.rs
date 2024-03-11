//! Outputs some metadata about a file.
//!
//! Usage: `cargo run --example avi_meta AVIFILE`

extern crate avirus;
use std::fs;

use avirus::AVI;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} AVIFILE", args[0]);
        std::process::exit(1);
    }
    let p = &args[1];
    let content = fs::read(p).expect("Unable to read AVI file.");
    let avi = AVI::new(&content).expect("Unable to read AVI file. Error");
    println!("{:?}", avi.header);
}
