
// mod file_manager;
pub mod file_manager;
pub mod log_manager;

use std::{io::*, sync::{Arc, Mutex}};

use self::{file_manager::FileManager, log_manager::LogManager};

static WORK_DIR: &str = "Database";
static BLOCK_SIZE: u64 = 500; // bytes 

struct Application {
    // fm: Arc<Mutex<FileManager>>,
    // lm: Arc<Mutex<LogManager>>
}

impl Application {

    // pub fn new(db_name:&str, block_size: u64) -> Result<Application>{

    //     let f = 
    //         FileManager::new(db_name,block_size)?;
        
    //     let fm = Arc::new(Mutex::new(f));
    //     let fm_clone = fm.clone();
    //     let log = db_name.to_string() + "_log";
    //     let lm = LogManager::new(fm_clone,&log)?;

    //     Ok(Application{
    //         fm:fm,
    //         lm:Arc::new(Mutex::new(lm)),
    //     })
    // }

}

