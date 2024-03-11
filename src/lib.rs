#![no_std]
extern crate byteorder;

pub mod cursor;
pub mod frame;
pub mod frames;
pub mod header;

use self::frames::Frames;
use byteorder::{ByteOrder, LittleEndian};
use cursor::{Cursor, Seek};
use header::Header;

/// The `AVI` type.
pub struct AVI<'a> {
    /// A Frames object. See [Frames](frames/struct.Frames.html) for more.
    pub frames: Frames<'a>,
    pub header: Header<'a>,
}

impl<'a> AVI<'a> {
    /// Loads a new `IoResult<AVI>` from an AVI file.
    ///
    /// # Examples
    /// ```
    /// use avirus::AVI;
    ///
    /// let mut avi = AVI::new("path_to.avi").unwrap();
    /// ```
    ///
    /// # Errors
    /// Several possible IO-related errors may be encountered in this function.
    /// * if `filename` does not already exist, see [`OpenOptions::open`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open) for more details
    /// * if a read error occurs during the reading of `filename`, see [`io::Read::read`](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) for more details
    /// * if expected headers in the byte stream are not found, [`io::ErrorKind::InvalidData`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidData) will be encountered
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if !is_formatted(data) {
            return None;
        }
        let mut rdr = Cursor::new(&data);
        let mut pos_of_header: usize = 0;
        rdr.seek(Seek::Start(12));
        let mut buf = [0u8; 4];
        rdr.read_exact(&mut buf);
        while buf == *b"LIST" || buf == *b"JUNK" {
            rdr.read_exact(&mut buf);
            let s = LittleEndian::read_u32(&buf);
            rdr.read_exact(&mut buf);
            if buf == *b"hdrl" {
                pos_of_header = rdr.position() as usize + 4;
            }
            rdr.seek(Seek::Current(s as isize - 4));
            rdr.read_exact(&mut buf);
        }
        rdr.read_exact(&mut buf);
        let s = LittleEndian::read_u32(&buf) + rdr.position() as u32;
        let frames = Frames::new(&data[rdr.position() as usize..s as usize]);
        let header = Header::new(&data[pos_of_header..pos_of_header + 11 * 4]);
        Some(Self { frames, header })
    }
}

fn is_formatted(data: &[u8]) -> bool {
    let mut reader = Cursor::new(&data);
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf);
    if buf != *b"RIFF" {
        return false;
    }
    reader.seek(Seek::Current(4));
    reader.read_exact(&mut buf);
    if buf != *b"AVI " {
        return false;
    }
    reader.read_exact(&mut buf);
    while buf == *b"LIST" || buf == *b"JUNK" {
        reader.read_exact(&mut buf);
        let s = LittleEndian::read_u32(&buf);
        reader.seek(Seek::Current(s as isize));
        reader.read_exact(&mut buf);
    }
    if buf != *b"idx1" {
        return false;
    }
    true
}
