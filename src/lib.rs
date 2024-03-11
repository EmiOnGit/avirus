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

/// The `Avi` type.
pub struct Avi<'a> {
    /// A Frames object. See [Frames](frames/struct.Frames.html) for more.
    pub frames: Frames<'a>,
    /// A Header object. See [Header](header/struct.Header.html) for more.
    pub header: Header<'a>,
}

impl<'a> Avi<'a> {
    /// Creates a new Avi.
    /// Returns `None` if the data is not a valid avi file
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
