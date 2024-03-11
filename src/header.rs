/// The `Header` type. This type defines global information in a AVI file.
/// For more information see [the official specification](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/api/Aviriff/ns-aviriff-avimainheader)
#[derive(Debug, Clone)]
pub struct Header {
    /// Specifies the number of microseconds between frames. This value indicates the overall timing for the file.
    pub microseconds_per_frame: u32,
    /// Specifies the approximate maximum data rate of the file.
    /// This value indicates the number of bytes per second the system must handle to present an AVI sequence as specified by the other parameters contained in the main header and stream header chunks.
    pub max_bytes_per_second: u32,
    /// Specifies the alignment for data, in bytes. Pad the data to multiples of this value.
    pub padding_granularity: u32,
    /// The following flags are defined:
    /// * `AVIF_COPYRIGHTED`: Indicates that the file uses copyrighted data and software.
    /// * `AVIF_HASINDEX`: Indicates that the file has an index.
    /// * `AVIF_ISINTERLEAVED`: Indicates that the file is interleaved.
    /// * `AVIF_MUSTUSEINDEX`: Indicates that application should use the index, rather than the physical ordering of the chunks in the file, to determine the order of presentation of the data.
    /// * `AVIF_WASCAPTUREFILE`: Indicates the AVI file is a specially allocated file used for capturing real-time video.
    pub flags: u32,
    /// Specifies the total amount of frames in the file.
    pub total_frames: u32,
    /// Specifies the initial frame for interleaved files. Noninterleaved files should specify zero.
    pub initial_frames: u32,
    /// Specifies the number of streams in the file.
    pub number_of_streams: u32,
    /// Specifies the suggested buffer size for reading the file.
    pub suggested_buffer_size: u32,
    /// Specifies the width of the AVI file in pixels.
    pub width: u32,
    /// Specifies the height of the AVI file in pixels.
    pub height: u32,
}
