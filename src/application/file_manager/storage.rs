#[path="page.rs"]
mod page; 
#[path="block.rs"]
mod block;

use std::{fs::OpenOptions, io::Result};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

use positioned_io_preview as positioned_io;
use positioned_io::{ReadAt,WriteAt,RandomAccessFile};

use super::block::Block;

#[derive(Debug)]
pub (super) struct Storage {
    
    db_dir: PathBuf,
    block_size: u64,
    storage: HashMap<String,RandomAccessFile>

}

impl Storage {

    pub (super) fn new(db_path:&str, block_size: u64) -> Result<Storage> {

        let mut db_dir = PathBuf::new();
            // let storage:CHashMap<String,Mutex<RandomAccessDisk>> = CHashMap::new();
        let storage:HashMap<String,RandomAccessFile> = HashMap::new();
        db_dir.push(db_path);

        if !db_dir.exists() {
            fs::create_dir(&db_path)?;
        }

        Ok(Storage {
            db_dir:db_dir,
            block_size:block_size,
            // storage: CHashMap::new(),
            storage:storage,
        })
    }

    pub (super) fn read(&mut self,file:&str, buffer: &mut[u8], offset:u64) -> Result<()>{

        self.db_dir.push(file);

        if !self.storage.contains_key(file){

            let new_file = OpenOptions::new().
                write(true).create(true).read(true).open(self.db_dir.as_path())?;

            let raf_wrapper = RandomAccessFile::try_new(new_file)?;
            self.storage.insert(String::from(file), raf_wrapper);
        }

        let raf = self.storage.get(file).unwrap();

        raf.read_at(offset*self.block_size, buffer);

        self.db_dir.pop();

        Ok(())
    }

    pub (super) fn write(&mut self,file:&str, buffer: &mut[u8], offset:u64) -> Result<()>{

        self.db_dir.push(file);

        if !self.storage.contains_key(file){

            let new_file = OpenOptions::new().
                write(true).create(true).read(true).open(self.db_dir.as_path())?;

            let raf_wrapper = RandomAccessFile::try_new(new_file)?;
            self.storage.insert(String::from(file), raf_wrapper);
        }

        let raf = self.storage.get_mut(file).unwrap();

        raf.write_at(offset*self.block_size, buffer);

        self.db_dir.pop();

        Ok(())
    }

    pub (super) fn length(&mut self,file:&str) -> Result<u64>{

        self.db_dir.push(file);

        if !self.storage.contains_key(file){

            let new_file = OpenOptions::new().
                write(true).create(true).read(true).open(self.db_dir.as_path())?;

            let raf_wrapper = RandomAccessFile::try_new(new_file)?;
            self.storage.insert(String::from(file), raf_wrapper);
        }

        let raf = self.storage.get(file).unwrap();

        let length = fs::metadata(self.db_dir.as_path())?.len();

        self.db_dir.pop();
         
        Ok(length)
    }

    pub (super) fn block(&mut self, file: &str) -> Result<Block>{

        self.db_dir.push(file);

        if !self.storage.contains_key(file){

            let new_file = OpenOptions::new().
                write(true).create(true).read(true).open(self.db_dir.as_path())?;

            let raf_wrapper = RandomAccessFile::try_new(new_file)?;
            self.storage.insert(String::from(file), raf_wrapper);
        }


        let raf = self.storage.get(file).unwrap();

        let length = fs::metadata(self.db_dir.as_path())?.len();

        let block_id = length / self.block_size;

        let new_blcok = Block::new(file,block_id);

        self.db_dir.pop();

        Ok(new_blcok)


    }




}

