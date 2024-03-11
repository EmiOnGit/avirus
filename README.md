
# avirus
This is a fork of the https://github.com/chinatsu/avirus repository.

A lot of internals have been rewritten to enable `no_std` and `no_alloc` support.


```toml,ignore
[dependencies]
avirus = { git="https://github.com/EmiOnGit/avirus" }
```

## Examples

`avirus::Avi` takes an byte buffer containing a valid avi file and allows direct read operations on the data without any extra allocations.
```rust

use avirus::Avi;

fn main() {
    let content: Vec<u8> = fs::read("sample_file.avi").expect("Unable to read file.");
    // Create a `Avi` struct by referencing a buffer.
    // Can fail if the buffer doesn't contain valid data.
    let avi = Avi::new(&content).unwrap();
    // The header of a avi contains meta informations.
    println!("frame count: {}", avi.header.total_frames());
    // Iteration over the frames is also possible.
    for frame in &avi.frames {
        println!("bytes of frame: {}", frame.length());
    }
}
```
