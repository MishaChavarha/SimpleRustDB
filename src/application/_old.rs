extern crate lazy_static;

pub mod block;
#[path="page.rs"]
pub mod page;

use std::{fs::OpenOptions, io::Result};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
// use std::sync::Mutex;
use std::fs;

use page::Page;

use positioned_io_preview as positioned_io;
use positioned_io::{ReadAt,WriteAt};

use super::WORK_DIR;
use block::Block;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref FILE_MANAGER: Option<Arc<Mutex<FileManager>>> =
        None;
}

pub struct FileManager {

    block_size: u64,
    db_path: PathBuf,
    files: HashMap<String,File>
    
}

impl FileManager {

    pub fn new(db_name:&str,block_size:u64) -> Result<FileManager>{

        let mut db_path = PathBuf::new();

        db_path.push(WORK_DIR);
        db_path.push(db_name);
    
        if !db_path.exists() {
            print!("1234");
            fs::create_dir(&db_path)?;
        }

        Ok(
            FileManager{
                block_size: block_size,
                db_path: db_path,
                files: HashMap::new()
            }
        )
    }

    pub fn read(&mut self, block:&Block, page: &mut Page) -> Result<()>{

        self.db_path.push(block.file());
        let path = self.db_path.as_path();

        // let mut files = self.files.lock().unwrap();

        // let mut files = self.files;

        let entry = self.files.entry(String::from(block.file())).or_insert_with(|| {
            OpenOptions::new().write(true).create(true).read(true)
                .open(path).unwrap()});
        
        entry.read_at(self.block_size*block.id(), page.buffer())?; 
        
        self.db_path.pop();

        Ok(())
    }

    pub fn write(&mut self, block:&Block, page: &mut Page) -> Result<()>{

        self.db_path.push(block.file());
        let path = self.db_path.as_path();

        // let mut files = self.files.lock().unwrap();

        let entry = self.files.entry(String::from(block.file())).or_insert_with(|| {
            OpenOptions::new().write(true).create(true).read(true)
                .open(path).unwrap()});
        
        entry.write_at(self.block_size*block.id(), page.buffer())?;      
        
        self.db_path.pop();

        Ok(())
    }

    // creates new block inside a file; 
    pub fn block(&mut self, file: &str) -> Result<Block> {

        self.db_path.push(file);
        let path = self.db_path.as_path();

        // let mut files = self.files.lock().unwrap();

        let entry = self.files.entry(String::from(file)).or_insert_with(|| {
            OpenOptions::new().write(true).create(true).read(true)
                .open(path).unwrap()});

        let length = fs::metadata(self.db_path.as_path()).unwrap().len();

        let block_number = length / self.block_size;

        let mut buffer = Vec::with_capacity(self.block_size as usize);
        entry.read_at(block_number*self.block_size, buffer.as_mut_slice())?;

        self.db_path.pop();

        Ok(Block::new(file,block_number))

    }
}

#[cfg(test)]
mod test {
    use std::fs;
    // use crate::file_manager::{block_id::BlockId, page::*};

    use super::{FileManager,Page};

    static BLOCK_SIZE: u64 = 500; // bytes
    static DB_DIR: &str = "test";

    // creates a database directory FMTEST and then deletes it; 

    #[test]
    fn test_cnstr_new_dir(){

        // creates and delets dB directory;
        let fm = FileManager::new(DB_DIR,BLOCK_SIZE).unwrap();
        fs::remove_dir_all(fm.db_path.as_path()).unwrap();
  
    }


    #[test]
    fn test_file_manager(){

        let mut  fm = FileManager::new(DB_DIR,BLOCK_SIZE).unwrap();
        let mut page1 = Page::new(BLOCK_SIZE);
        let block = &fm.block("testname").unwrap();

        let offset = 0; 

        page1.write_bytes(offset, "1234567890".as_bytes()).unwrap();

        // save page1 into the file
        fm.write(block, &mut page1).unwrap();

         // define another page; 
        let mut page2 = Page::new(500);
        fm.read(block, &mut page2).unwrap();

        assert_eq!(page2.read_bytes(offset).unwrap(),"1234567890".as_bytes());

        println!("bytes correctly read ");
    }
}
