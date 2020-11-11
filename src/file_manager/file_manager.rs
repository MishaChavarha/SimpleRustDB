use std::{fs::{DirBuilder, File}, sync::Mutex};
use std::io::{Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use positioned_io_preview as positioned_io;
use positioned_io::{RandomAccessFile, ReadAt};

use crate::{BlockId, Page};

struct FileManager {

    directory: PathBuf,
    is_new: bool,
    block_size: u64,
    // so far leave it like this so that only one random disk access is allowed
    open_files: Mutex<HashMap<String,RandomAccessFile>>

}

impl FileManager {
    pub fn new(dir_path: PathBuf, block_size: u64) -> Result<FileManager> {

        assert_eq!(dir_path.is_dir(),true);

        let mut is_new = false;

        // directory path is delcared in a separate application class
        if !dir_path.is_dir() {
            fs::create_dir(&dir_path)?;
            is_new = true;
        }

        let open_files = Mutex::new(HashMap::new());

        // TODO: 
        // remove files that start with temp

        Ok(
            FileManager {
                directory: dir_path,
                is_new: is_new,
                block_size: block_size,
                open_files: open_files
            }
        )
    }



    pub fn read(&self, block: &BlockId, page: &mut Page) -> Result<()>{

        let mut open_files = self.open_files.lock().unwrap();
        // TO DO: 
        // perform checks
        self.create_if_new(&mut open_files, block.file())?;
        let raf = open_files.get(block.file()).unwrap();
        let result = raf.read_at(block.number()*self.block_size, page.contents());



        // let file = self.open_files.lock().unwrap().get(block.file_name()).unwrap_or_else(new_file(block.file_name()));

        return Ok(());
    } 


    pub (self) fn create_if_new(&self, open_files: & mut HashMap<String,RandomAccessFile>, filestr: &str) -> Result<()>{

        if !open_files.contains_key(filestr) {

            let mut pathbuff: PathBuf = PathBuf::new();
            pathbuff.push(self.directory.as_path());  
            pathbuff.push(filestr);
            // create new file 
            let file = File::create(&pathbuff.as_path())?;
            let raf = RandomAccessFile::try_new(file)?;


            open_files.insert(String::from(filestr), raf);
        }

        return Ok(());
    } 
    

}