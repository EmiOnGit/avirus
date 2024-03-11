/// The `Frames` type.
pub struct Frames<'a> {
    pub data: &'a [u8],
}

impl<'a> Frames<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}
