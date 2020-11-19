extern crate lazy_static;

pub mod block;
pub mod page;

mod storage;

use std::{fs::OpenOptions, io::Result};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::*;
use page::Page;

use random_access_disk::RandomAccessDisk;

use self::storage::Storage;

use super::WORK_DIR;
use block::Block;
use std::sync::{Arc, Mutex};



#[derive(Debug)]
pub struct FileManager {

    fm: Arc<Mutex<Storage>>

    // storage: CHashMap<String,Mutex<V>>
    
}

impl FileManager {

    pub fn new(db_path:&str, block_size: u64) -> Result<FileManager>{
        let fm = 
            Storage::new(db_path, block_size)?;

        Ok(FileManager {
            fm:Arc::new(Mutex::new(fm)),
        })
    }

    pub fn read(&self, block: &Block, page: &mut Page) -> Result<()> {
        
        let mut storage = self.fm.lock().unwrap();
        let file = block.file();
        let offset = block.id();
        let buffer = page.buffer();
        
        storage.read(file,buffer,offset)?;

        Ok(())
    }
    
    pub fn write(&self, block: &Block, page: &mut Page) -> Result<()>{

        let mut storage = self.fm.lock().unwrap();
        let file = block.file();
        let offset = block.id();
        let buffer = page.buffer();
        
        storage.write(file,buffer,offset)?;

        Ok(())
    }

    pub fn length(&self, file:&str) -> Result<u64> {

        self.length(file)

    }

    pub fn block(&self, file:&str) -> Result<Block> {

        let mut storage = self.fm.lock().unwrap();
        let block = storage.block(file)?;


        return Ok(block);
    }

}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, fs};
    // use crate::file_manager::{block_id::BlockId, page::*};

    use crate::application::WORK_DIR;

    use super::{FileManager,Page};

    static BLOCK_SIZE: u64 = 500; // bytes
    static DB_DIR: &str = "test";

    // creates a database directory FMTEST and then deletes it; 

    // #[test]
    // fn test_cnstr_new_dir(){

    //     // creates and delets dB directory;
    //     let fm = FileManager::new(DB_DIR,BLOCK_SIZE).unwrap();
    //     fs::remove_dir_all(fm.db_path.as_path()).unwrap();
  
    // }


    #[test]
    fn test_file_manager(){

        let mut db_dir = PathBuf::new();
        db_dir.push(WORK_DIR);
        db_dir.push(DB_DIR);

        let mut fm = FileManager::new(db_dir.as_path().to_str().unwrap(),BLOCK_SIZE).unwrap();
        let mut page1 = Page::new(BLOCK_SIZE);
        let block = fm.block("testname").unwrap();

        let offset = 0; 

        page1.write_bytes(offset, "1234567890".as_bytes()).unwrap();

        // save page1 into the file
        fm.write(&block, &mut page1).unwrap();

         // define another page; 
        let mut page2 = Page::new(500);
        fm.read(&block, &mut page2).unwrap();

        assert_eq!(page2.read_bytes(offset).unwrap(),"1234567890".as_bytes());

        println!("bytes correctly read ");
    }
}
