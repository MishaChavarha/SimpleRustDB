use std::{fs::DirBuilder, sync::Mutex};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use positioned_io_preview as positioned_io;
use positioned_io::{RandomAccessFile};

struct FileManager {

    directory: PathBuf,
    isnew: bool,
    blocksize: u64,

    // so far leave it like this so that only one random disk access is allowed
    openfiles: Mutex<HashMap<String,RandomAccessFile>>

}

impl FileManager {

    pub fn new(path: PathBuf, blocksize: u64) {

        let mut is_new = false;
        // directory exists
        if !path.is_dir(){
            fs::create_dir(path);
            is_new = true;
        }




        // steps 
        // 1 chceck if directory exists; 


    }
    
}