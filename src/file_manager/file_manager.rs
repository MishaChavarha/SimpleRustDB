use std::{fs::{DirBuilder, File, OpenOptions}, sync::Mutex};
use std::io::{Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use positioned_io_preview as positioned_io;
use positioned_io::{RandomAccessFile, ReadAt, WriteAt};

use crate::file_manager::page::*;
use crate::file_manager::block_id::*;

struct FileManager {

    directory: PathBuf,
    is_new: bool,
    block_size: u64,
    // so far leave it like this so that only one disk access is allowed
    // perhaps change it to HashMap of Mutexes;
    open_files: Mutex<HashMap<String,File>>,
    blank_buffer: Vec<u8>,

}

impl FileManager {
    pub fn new(dir_path: PathBuf, block_size: u64) -> Result<FileManager> {

        // assert_eq!(dir_path.is_dir(),true);

        let mut is_new = false;

        // directory path is delcared in a separate application class
        if !dir_path.is_dir() {
            fs::create_dir(&dir_path)?;
            is_new = true;
        }

        let open_files = Mutex::new(HashMap::new());
        let blank_buffer = vec![0 as u8;block_size as usize];
    
        // TODO: 
        // remove files that start with temp

        Ok(
            FileManager {
                directory: dir_path,
                is_new: is_new,
                block_size: block_size,
                open_files: open_files,
                blank_buffer: blank_buffer
            }
        )
    }

    pub fn directory(&self) -> &Path {
        return self.directory.as_path();
    }



    pub fn read(&self, block: &BlockId, page: &mut Page) -> Result<()>{

        let mut open_files = self.open_files.lock().unwrap();
        // TO DO: 
        // perform checks
        self.create_if_new(&mut open_files, block.file())?;
        let raf = open_files.get(block.file()).unwrap();
        let innervec = page.contents();
        raf.read_at(block.number()*self.block_size, innervec)?;

        // let file = self.open_files.lock().unwrap().get(block.file_name()).unwrap_or_else(new_file(block.file_name()));

        return Ok(());
    } 

    pub fn write(&self, block: &BlockId, page: &mut Page) -> Result<()>{

        let mut open_files = self.open_files.lock().unwrap();

        // TO DO: 
        // perform checks
        self.create_if_new(&mut open_files, block.file())?;
        let mut raf = open_files.get_mut(block.file()).unwrap();
        raf.write_at(block.number()*self.block_size, page.contents())?;

        return Ok(());
         
    }

    pub fn get_new_block(&mut self, file_name: &str) -> Result<BlockId> {
        
        let mut open_files = self.open_files.lock().unwrap();
        let block_number = self.get_file_size(&mut open_files, file_name)?;
        open_files.get(file_name).unwrap().read_at(block_number*self.block_size, self.blank_buffer.as_mut_slice())?;

        return Ok(BlockId::new(String::from(file_name),block_number));
    }


    pub (self) fn create_if_new(&self, open_files: & mut HashMap<String,File>, filestr: &str) -> Result<()>{


        if !open_files.contains_key(filestr) {

            let mut path: PathBuf = PathBuf::new();
            path.push(self.directory.as_path());  
            path.push(filestr);

            // create new file 
            let file = OpenOptions::new().write(true).create(true).read(true).open(path)?;
            // let raf = RandomAccessFile::try_new(file)?;
            open_files.insert(String::from(filestr), file);
        }
        return Ok(());
    }

    fn get_file_size(&self,open_files:&mut HashMap<String,File>, file: &str) -> Result<u64>{

        self.create_if_new(open_files, file)?;

        let size = File::metadata(open_files.get(file).unwrap()).unwrap().len();
        // let size = File::metadata(&file)?.len();
       
        return Ok( size / self.block_size as u64);

    }
}


#[cfg(test)]
mod test {
    use std::{fs, path::{Path, PathBuf}};
    use crate::file_manager::{block_id::BlockId, page::*};

    use super::FileManager;

    static TEST_DIRECTORY: &str = "Database/Test/";
    static BLOCK_SIZE: u64 = 500; // bytes

    // creates a database directory FMTEST and then deletes it; 

    #[test]
    fn test_cnstr_new_dir(){

        let mut path = PathBuf::from(Path::new(TEST_DIRECTORY));
        let database = "FMTEST";
        path.push(database);
        // assert_eq!(dir.is_dir(), true);
        let fm = FileManager::new(PathBuf::from(path),100).unwrap();
        assert_eq!(fm.directory().is_dir(),true);

        fs::remove_dir_all(fm.directory()).unwrap();
        assert_ne!(fm.directory().is_dir(),true);
  
    }

    #[test]
    fn test_file_manager(){
         let mut path = PathBuf::from(Path::new(TEST_DIRECTORY));
         let database = "FMTEST";
         path.push(database);

         let mut fm = FileManager::new(PathBuf::from(path),BLOCK_SIZE).unwrap();
         let mut page1 = Page::new(BLOCK_SIZE);
         let block = fm.get_new_block("testname").unwrap();

         // define offset in the page to 100 bytes; 
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