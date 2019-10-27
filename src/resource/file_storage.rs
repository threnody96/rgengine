use std::path::PathBuf;
use ::util::{ exe_dir, load_file };

const SEPARATOR: char = '/';

pub struct FileStorage { }

impl FileStorage {

    pub fn new() -> Self {
        Self {}
    }

    fn resource_path() -> PathBuf {
        let mut storage_dir = exe_dir();
        storage_dir.push("resource");
        storage_dir
    }

    pub fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let mut fullpath = Self::resource_path();
        for p in path.split(SEPARATOR) {
            fullpath.push(p);
        }
        load_file(&fullpath)
    }

}

