use byteorder::{BigEndian, ByteOrder, LittleEndian};

const AVIIF_KEYFRAME: u32 = 0x0000_0010;

/// The `Frame` type. This is the lowest level type
/// in the AVI file, ignoring the codec level.
#[derive(Clone, Copy, Debug)]
pub struct Frame<'a> {
    data: &'a [u8; 16],
}
impl<'a> Frame<'a> {
    /// This function reads a `&[u8]`, reads four chunks of 4 and returns a `Frame`.
    /// It is expected that `id`, `flag`, `offset`, and `length` are in the correct order
    /// in the input file above.
    #[must_use]
    pub fn new(bytes: &'a [u8; 16]) -> Self {
        Self { data: bytes }
    }
    /// Specifies a four-character code corresponding to the chunk ID of a data chunk in the file. See [stream data ('movi' list)](https://docs.microsoft.com/en-us/windows/desktop/directshow/avi-riff-file-reference#stream-data-movi-list) for more information.
    pub fn id(&self) -> u32 {
        BigEndian::read_u32(&self.data[0..4])
    }
    /// The following flags are defined:
    /// * `AVIIF_KEYFRAME`: The chunk the entry refers to is a keyframe
    /// * `AVIIF_LIST`: The entry points to a list, not a chunk
    /// * `AVIIF_FIRSTPART`: Indicates this chunk needs the frames following it to be used; it cannot stand alone
    /// * `AVIIF_LASTPART`: Indicates this chunk needs the frames preceding it to be used; it cannot stand alone
    /// * `AVIIF_NOTIME`: The duration which is applied to the corresponding chunk is 0
    pub fn flag(&self) -> u32 {
        LittleEndian::read_u32(&self.data[4..8])
    }
    /// Contains the position of the header of the corresponding chunk
    pub fn offset(&self) -> u32 {
        LittleEndian::read_u32(&self.data[8..12])
    }
    /// Contains the size of the corresponding chunk in bytes
    pub fn length(&self) -> u32 {
        LittleEndian::read_u32(&self.data[12..16])
    }
    /// This function outputs the `Frame` as a `[u8; 16]`.

    /// This function returns a boolean which indicates that this frame is a video frame.
    #[must_use]
    pub fn is_videoframe(&self) -> bool {
        let id = self.id_as_u8_array();
        &id[2..4] == b"db" || &id[2..4] == b"dc"
    }

    /// This function returns a boolean which indicates that this frame is an audio frame.
    #[must_use]
    pub fn is_audioframe(&self) -> bool {
        let id = self.id_as_u8_array();
        &id[2..4] == b"wb"
    }

    /// This function returns a boolean which indicates that this frame is a key frame
    /// (hereby known as an iframe).
    #[must_use]
    pub fn is_iframe(&self) -> bool {
        if self.is_videoframe() {
            return self.flag() & AVIIF_KEYFRAME != 0;
        }
        false
    }

    /// This function returns a boolean which indicates that this frame is a delta frame
    /// (hereby known as a pframe).
    #[must_use]
    pub fn is_pframe(&self) -> bool {
        if self.is_videoframe() {
            return self.flag() & AVIIF_KEYFRAME == 0;
        }
        false
    }

    /// This is a private function to cast a `u32` to a `[u8; 4]`.
    fn id_as_u8_array(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, self.id());
        buf
    }
}
