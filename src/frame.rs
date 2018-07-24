use byteorder::{ByteOrder, BigEndian, LittleEndian};


const AVIIF_KEYFRAME: u32 = 0x00000010;

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub id: u32,
    pub flag: u32,
    pub offset: u32,
    pub length: u32
}

impl Frame {
    pub fn new(bytes: &[u8]) -> Frame {
        let mut iter = bytes.chunks(4);
        Frame {
            id: BigEndian::read_u32(iter.next().unwrap()),
            flag: LittleEndian::read_u32(iter.next().unwrap()),
            offset: LittleEndian::read_u32(iter.next().unwrap()),
            length: LittleEndian::read_u32(iter.next().unwrap()),
        }
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        BigEndian::write_u32_into(&[self.id, self.flag, self.offset, self.length], &mut buf);
        buf
    }

    pub fn is_videoframe(&self) -> bool {
        let id = self.id_as_u8_array();
        &id[2..4] == b"db" || &id[2..4] == b"dc"
    }

    pub fn is_audioframe(&self) -> bool {
        let id = self.id_as_u8_array();
        &id[2..4] == b"wb"
    }

    pub fn is_iframe(&self) -> bool {
        if self.is_videoframe() {
            return self.flag & AVIIF_KEYFRAME != 0;
        }
        false
    }

    pub fn is_pframe(&self) -> bool {
        if self.is_videoframe() {
            return self.flag & AVIIF_KEYFRAME == 0;
        }
        false
    }

    fn id_as_u8_array(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, self.id);
        buf
    }
}