use std::io::*;

struct Page {
    buffer: Cursor<Vec<u8>>
}

impl Page {
    pub fn new(buffer_size: usize) -> Page{
        Page{
            buffer: Cursor::new(Vec::with_capacity(buffer_size))
        }
    }
}