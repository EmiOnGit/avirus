pub(crate) struct Cursor<T> {
    data: T,
    pointer: usize,
}
impl<T> Cursor<T> {
    pub fn new(data: T) -> Self {
        Cursor { data, pointer: 0 }
    }
    pub fn position(&self) -> usize {
        self.pointer
    }
}
impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    pub fn seek(&mut self, seek: Seek) {
        self.pointer = match seek {
            Seek::Current(s) => self
                .pointer
                .checked_add_signed(s)
                .expect("Invalid seek leading to a overflow"),
            Seek::Start(s) => s,
        }
    }
    pub fn read_exact(&mut self, buffer: &mut [u8]) {
        let data = self.data.as_ref();
        for i in 0..buffer.len() {
            buffer[i] = data[self.pointer];
            self.pointer += 1;
        }
    }
}
#[derive(Clone, Copy)]
pub(crate) enum Seek {
    Current(isize),
    Start(usize),
}
