extern crate byteorder;
use byteorder::{LittleEndian,ReadBytesExt,WriteBytesExt};
use std::io::*;

pub struct Page {
    buffer: Cursor<Vec<u8>>,
}

impl Page {
    pub fn new(buffer_size: usize) -> Page{
        return 
        Page{
            buffer: Cursor::new(Vec::with_capacity(buffer_size)),
        }
    }

    pub fn write_int32(&mut self, offset: u64, number: i32)->Result<()>{
        
        self.buffer.set_position(offset as u64);

        return self.buffer.write_i32::<LittleEndian>(number);

    }

    pub fn get_int32(&mut self, offset: u64) -> Result<i32> {

        self.buffer.set_position(offset);

        return self.buffer.read_i32::<LittleEndian>();
    }

    // pub fn write_bytes(&mut self, offset: u64, buffer: &[u8]) -> Result<()>{

    //     self.buffer.set_position(offset);

    //     let write_result = self.buffer.write(buffer);
    //     match write_result {
    //         Err(e) => return Err(e),
    //         // add checks to make sure correct number of bytes were written
    //         Ok(_) => return Ok(())
    //     }

    // }

    // pub fn get_bytes(&mut self, offset: u64) -> Result<&[u8]> {

    //     self.buffer.set_position(offset);

    //     let buffer_size = self.buffer.read_u64::<LittleEndian>();
    //     match buffer_size {
    //         Err(e) => return Err(e),
    //         Ok(_) => (),
    //     }

    //     let start_pos = self.buffer.position() as usize;
    //     let end_pos = buffer_size.unwrap() as usize;

    //     if(buffer_size )

    //     return Ok(&self.buffer.get_mut()[start_pos..end_pos]);
    // }
}