use std::sync::{Arc, Mutex};
use std::io::Result;

use super::file_manager::{block::Block, FileManager, page::Page};

pub struct LogManager {
    // file_manager: Arc<Mutex<FileManager>>,
    // log_page: Page,
    // log_file: String,
    // curr_block: Block,
    // latest_lvn: u64, 
    // last_saved_lvn: u64


}

impl LogManager {
    
    // pub fn new(fm: Arc<Mutex<FileManager>>, log_file:&str) -> Result<LogManager>{
    //     // let fm = fm.clone();
    //     let fm_clone = fm.clone();
    //     let mut file_manager = fm_clone.lock().unwrap();
    //     let block_size = file_manager.block_size();
    //     let curr_block;
    //     let mut log_page = Page::new(file_manager.block_size());
    //     let log_size = file_manager.file_length(log_file);
    //     match  log_size {
    //         0 => curr_block = file_manager.block(log_file)?,
    //         num => {
    //             curr_block = Block::new(log_file,log_size-1);
    //             file_manager.read(&curr_block, &mut log_page );
    //         } 
    //     }

    //     let latest_lsn = 0;
    //     let last_saved_lsn = 0;
    //     let log_file = String::from(log_file);


    //     Ok(LogManager{

    //         file_manager:fm.clone(),
    //         log_page:log_page,
    //         log_file:log_file,
    //         curr_block:curr_block,
    //         latest_lvn:latest_lsn,
    //         last_saved_lvn:last_saved_lsn,

    //     })
    // }

    
}
