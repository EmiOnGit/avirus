//! Outputs some metadata about a file.
//!
//! Usage: `cargo run --example avi_meta AVIFILE`

extern crate avirus;
use std::fs;

use avirus::Avi;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} AVIFILE", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let content = fs::read(path).expect("Unable to read file.");
    let avi = Avi::new(&content).expect("no valid AVI file");
    println!("header: {:?}", avi.header);
    println!("width: {}", avi.header.width());
    println!("height: {}", avi.header.height());
    println!("frame count : {}", avi.header.total_frames());
    for frame in &avi.frames {
        println!("bytes of frame: {}", frame.length());
    }
}
