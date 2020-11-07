mod file_manager;

use file_manager::page::*;

use std::string::*;
use std::vec::*;
use file_manager::block_id::BlockId;
use std::io::Cursor;

fn main() {
    // let buffer: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(1024));

    let page = Page::new(400);

    let test: i32 = -324; 
    let array = test.to_le_bytes();
    
    print!("{:?} {:?}",test.to_le_bytes(), i32::from_le_bytes(array))

}
