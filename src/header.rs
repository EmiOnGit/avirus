use byteorder::ByteOrder;
use byteorder::LittleEndian;

/// The `Header` type. This type defines global information in a AVI file.
/// For more information see [the official specification](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/api/Aviriff/ns-aviriff-avimainheader)
#[derive(Debug, Clone)]
pub struct Header<'a> {
    data: &'a [u8],
}
impl<'a> Header<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { data: bytes }
    }
    fn read_little_endian(&self, index: usize) -> u32 {
        LittleEndian::read_u32(&self.data[index * 4..(index + 1) * 4])
    }
    /// Specifies the number of microseconds between frames. This value indicates the overall timing for the file.
    pub fn microseconds_per_frame(&self) -> u32 {
        self.read_little_endian(1)
    }
    /// Specifies the approximate maximum data rate of the file.
    /// This value indicates the number of bytes per second the system must handle to present an AVI sequence as specified by the other parameters contained in the main header and stream header chunks.
    pub fn max_bytes_per_second(&self) -> u32 {
        self.read_little_endian(2)
    }
    /// Specifies the alignment for data, in bytes. Pad the data to multiples of this value.
    pub fn padding_granularity(&self) -> u32 {
        self.read_little_endian(3)
    }
    /// The following flags are defined:
    /// * `AVIF_COPYRIGHTED`: Indicates that the file uses copyrighted data and software.
    /// * `AVIF_HASINDEX`: Indicates that the file has an index.
    /// * `AVIF_ISINTERLEAVED`: Indicates that the file is interleaved.
    /// * `AVIF_MUSTUSEINDEX`: Indicates that application should use the index, rather than the physical ordering of the chunks in the file, to determine the order of presentation of the data.
    /// * `AVIF_WASCAPTUREFILE`: Indicates the AVI file is a specially allocated file used for capturing real-time video.
    pub fn flags(&self) -> u32 {
        self.read_little_endian(4)
    }
    /// Specifies the total amount of frames in the file.
    pub fn total_frames(&self) -> u32 {
        self.read_little_endian(5)
    }
    /// Specifies the initial frame for interleaved files. Noninterleaved files should specify zero.
    pub fn initial_frames(&self) -> u32 {
        self.read_little_endian(6)
    }
    /// Specifies the number of streams in the file.
    pub fn number_of_streams(&self) -> u32 {
        self.read_little_endian(7)
    }
    /// Specifies the suggested buffer size for reading the file.
    pub fn suggested_buffer_size(&self) -> u32 {
        self.read_little_endian(8)
    }
    /// Specifies the width of the AVI file in pixels.
    pub fn width(&self) -> u32 {
        self.read_little_endian(9)
    }
    /// Specifies the height of the AVI file in pixels.
    pub fn height(&self) -> u32 {
        self.read_little_endian(10)
    }
}
