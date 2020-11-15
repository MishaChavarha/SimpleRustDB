
// mod file_manager;
pub mod file_manager;


use self::file_manager::FileManager;
use std::io::*;

struct Application {
    file_manager: FileManager
}

static WORK_DIR: &str = "Database/";
static BLOCK_SIZE: u64 = 400; // bytes 

pub fn application_run() -> Result<()> {

    FileManager::new("new_database",BLOCK_SIZE)?;

    return Ok(());
}


