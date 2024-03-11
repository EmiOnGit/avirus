use core::{convert::TryInto, slice::ChunksExact};

use crate::frame::{Frame, FRAME_SIZE};

/// The `Frames` of the avi.
/// use `Frames::iter` to iterate over the Frames
pub struct Frames<'a> {
    data: &'a [u8],
}

impl<'a> Frames<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            frames: self.data.chunks_exact(FRAME_SIZE),
        }
    }
}
impl<'a> IntoIterator for &'a Frames<'a> {
    type Item = Frame<'a>;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    frames: ChunksExact<'a, u8>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = Frame<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.frames
            .next()
            .map(|chunk| Frame::new(chunk.try_into().unwrap()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.frames.size_hint()
    }
}
