extern crate byteorder;

pub mod frame;
pub mod frames;
pub mod header;

use self::frames::Frames;
use byteorder::{ByteOrder, LittleEndian};
use header::Header;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Read;
use std::io::Result as IoResult;
use std::io::SeekFrom;
use std::io::{Error, ErrorKind};

/// The `AVI` type.
pub struct AVI {
    /// A Frames object. See [Frames](frames/struct.Frames.html) for more.
    pub frames: Frames,
    /// The byte stream, represented by a `Vec` of `u8`.
    pub stream: Vec<u8>,
    pub header: Header,
}

impl AVI {
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
    pub fn new(filename: &str) -> IoResult<Self> {
        let mut f = File::open(filename)?;
        let mut file_data: Vec<u8> = Vec::new();
        f.read_to_end(&mut file_data)?;
        is_formatted(&file_data)?;

        let mut rdr = Cursor::new(&file_data);
        let mut pos_of_movi: usize = 0;
        let mut pos_of_header: usize = 0;
        rdr.seek(SeekFrom::Start(12))?;
        let mut buf = [0u8; 4];
        rdr.read_exact(&mut buf)?;
        while buf == *b"LIST" || buf == *b"JUNK" {
            rdr.read_exact(&mut buf)?;
            let s = LittleEndian::read_u32(&buf);
            rdr.read_exact(&mut buf)?;
            if buf == *b"movi" {
                pos_of_movi = rdr.position() as usize - 4;
            }
            if buf == *b"hdrl" {
                pos_of_header = rdr.position() as usize + 4;
            }
            rdr.seek(SeekFrom::Current(i64::from(s) - 4))?;
            rdr.read_exact(&mut buf)?;
        }
        rdr.read_exact(&mut buf)?;
        let s = LittleEndian::read_u32(&buf) + rdr.position() as u32;
        let frames = Frames::new(&file_data[rdr.position() as usize..s as usize], pos_of_movi);
        let header = Header::new(&file_data[pos_of_header..pos_of_header + 11 * 4]);
        Ok(Self {
            frames,
            header,
            stream: file_data,
        })
    }

    /// Constructs a binary AVI file from an AVI type.
    ///
    /// # Examples
    /// ```
    /// use avirus::AVI;
    ///
    /// let ut avi = AVI::new("path_to.avi").unwrap();
    /// avi.output("path_to_new.avi").unwrap();
    /// ```
    ///
    /// # Errors
    /// Several possible IO-related errors may be encountered in this function.
    /// * if a reading error is encountered during the creation of framedata, see [`frames::Frames::make_framedata`](frames/struct.Frames.html#method.make_framedata) for more details
    /// * if an error is encountered during creation of the file, see [`io::File::create`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) for more details
    /// * if an writing error is encountered during output, see [`io::Write::write`](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write) for more details
    pub fn output(&mut self, filename: &str) -> IoResult<()> {
        let io = self.frames.make_framedata(&self.stream)?;
        self.overwrite(&io);
        let mut f = File::create(filename)?;
        f.write_all(&self.stream)?;
        Ok(())
    }
    /// A method which overwrites parts of `AVI::stream` with the input
    /// `framedata`. This is normally called automatically in `AVI::output` and
    /// uses the current state of `Frames`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn overwrite(&mut self, framedata: &[u8]) {
        let mut new_stream: Vec<u8> = Vec::new();
        new_stream.extend_from_slice(&self.stream[..self.frames.pos_of_movi - 4]);
        let mut buf = [0u8; 4];
        LittleEndian::write_u32_into(&[4u32], &mut buf);
        new_stream.extend_from_slice(&buf);
        new_stream.extend_from_slice(framedata);
        new_stream.extend_from_slice(b"idx1");
        LittleEndian::write_u32_into(&[self.frames.meta.len() as u32], &mut buf);
        new_stream.extend_from_slice(&buf);
        let mut framecount = 0u32;
        for frame in &self.frames.meta {
            new_stream.extend_from_slice(&frame.as_bytes());
            if frame.is_videoframe() {
                framecount += 1;
            }
        }
        let eof = new_stream.len() as u32;
        LittleEndian::write_u32_into(&[eof - 8], &mut buf);
        new_stream[4..7].copy_from_slice(&buf[..(7 - 4)]);

        LittleEndian::write_u32_into(&[framecount], &mut buf);
        new_stream[48..51].copy_from_slice(&buf[..(51 - 48)]);

        self.stream = new_stream;
    }
}

/// Validates AVI formatting of an input binary file.
/// This is a private function used internally during
/// `AVI::new()`.
///
/// # Errors
/// `io::ErrorKind::InvalidData` upon encountering a missing header at an expected position.
fn is_formatted(file: &Vec<u8>) -> IoResult<()> {
    let mut reader = Cursor::new(&file);
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    if buf != *b"RIFF" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Malformed AVI, missing RIFF at expected position 0x{:x}",
                reader.position()
            ),
        ));
    }
    reader.seek(SeekFrom::Current(4))?;
    reader.read_exact(&mut buf)?;
    if buf != *b"AVI " {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Malformed AVI, missing AVI at expected position 0x{:x}",
                reader.position()
            ),
        ));
    }
    reader.read_exact(&mut buf)?;
    while buf == *b"LIST" || buf == *b"JUNK" {
        reader.read_exact(&mut buf)?;
        let s = LittleEndian::read_u32(&buf);
        reader.seek(SeekFrom::Current(s.into()))?;
        reader.read_exact(&mut buf)?;
    }
    if buf != *b"idx1" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Malformed AVI, missing idx1 at expected position 0x{:x}",
                reader.position()
            ),
        ));
    }
    Ok(())
}
