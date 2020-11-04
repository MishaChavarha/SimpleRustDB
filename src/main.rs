mod file_manager;

use std::string::*;
use std::vec::*;
use file_manager::block_id::BlockId;
use std::io::Cursor;

fn main() {
    let buffer: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(1024));

}
