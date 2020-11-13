extern crate byteorder;

use byteorder::{LittleEndian,ReadBytesExt,WriteBytesExt};
use std::io::*;
#[derive(Debug)]
pub struct Page {
    buffer: Cursor<Vec<u8>>,
}

// Notes:
// i am not sure if read_int advances cursor. Do I need to manually offset it. 
// if data does not fit page size then it has to be offset. 

impl Page {
    pub fn new(buffer_size: u64) -> Page{

        let mut buffer = Cursor::new(vec![0 as u8;buffer_size as usize]);

        // assert_eq!(buffer.get_mut().len(),buffer_size as usize);
        Page{
            buffer: buffer,
        }
    }
    pub fn contents(&mut self) -> &mut [u8]{
        return self.buffer.get_mut().as_mut_slice();
    }

    pub fn write_int(&mut self, offset: u64, data: i32) -> Result<()>{
        
        self.buffer.set_position(offset as u64);
        return self.buffer.write_i32::<LittleEndian>(data);

    }

    pub fn read_int(&mut self, offset: u64) -> Result<i32> {

        self.buffer.set_position(offset);
        return self.buffer.read_i32::<LittleEndian>();
    }

    pub fn write_bytes(&mut self, offset: u64, data: &[u8]) -> Result<()>{

        self.buffer.set_position(offset);
        let size = data.len();
        self.buffer.write_u32::<LittleEndian>(size as u32);
        self.buffer.write(data);

        return Ok(());
    }

    pub fn read_bytes(&mut self, offset: u64) -> Result<&[u8]>{

        self.buffer.set_position(offset);
        let size = self.buffer.read_u32::<LittleEndian>().unwrap();
        println!("size readbytes {}", size);
        let pos = self.buffer.position() as usize;
        // assert_eq!(offset+4,pos as u64,"offfset {} and {} are equal", offset+4, pos);
        let slice = &self.buffer.get_ref()[pos..pos+size as usize];

        // check for errorrs when it does not fit the slice. 

        return Ok(slice);
    }

}

#[cfg(test)]
mod page_test {

    pub enum PageError {

    }

   use super::Page;

    #[test]
    fn test_int(){

        let buffer_size = 100; //bytes
        let mut page = Page::new(buffer_size);

        page.write_int(0, 10);
        let read_int = page.read_int(0);
        assert_eq!(10,read_int.unwrap());
        page.write_int(10, 100);
        let read_int = page.read_int(10);
        assert_eq!(100,read_int.unwrap());


    }

    #[test]
    fn test_bytes(){

        let buffer_size = 100;
        let mut page = Page::new(buffer_size);

        let test_data = "12345678910".as_bytes();
        page.write_bytes(10, test_data);
        let read_data = page.read_bytes(10);
        assert_eq!(test_data,read_data.unwrap());

        let test_data2 = String::from("123456789abcdefgh");
        page.write_bytes(12, test_data2.as_bytes());
        let read_data2 = page.read_bytes(12);
        assert_eq!(test_data2.as_bytes(),read_data2.unwrap());

        let test_data3 = String::from("123456789abcdefgh12");
        assert_ne!(test_data2.as_bytes(),test_data3.as_bytes());

        let test_string = String::from("12345");
        page.write_bytes(40, test_string.as_bytes());
        let read_string = String::from_utf8_lossy(page.read_bytes(40).unwrap());

        println!("line is {}", read_string)

    }
}