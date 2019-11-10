use std::path::PathBuf;
use std::fs::{ create_dir_all, File };
use std::io::{BufWriter, Write};
use ::util::{ exe_dir, load_file, DIR_SEPARATOR, decrypt, encrypt };

pub struct FileStorage {
    path: PathBuf,
    encrypt_key: Option<String>
}

impl FileStorage {

    pub fn new(path: &str, encrypt_key: Option<String>) -> Self {
        Self {
            path: Self::resource_path(path),
            encrypt_key: encrypt_key
        }
    }

    pub fn new_resource() -> Self {
        Self::new("resource", None)
    }

    fn resource_path(path: &str) -> PathBuf {
        let mut storage_dir = exe_dir();
        let paths: Vec<&str> = path.split(DIR_SEPARATOR).collect();
        for p in paths {
            storage_dir.push(p);
        }
        storage_dir
    }

    pub fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let data = load_file(&self.generate_file_path(path))?;
        if let Some(key) = &self.encrypt_key {
            decrypt(data.as_slice(), key.as_str())
        } else {
            Ok(data)
        }
    }

    pub fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        let p = self.generate_file_path(path);
        if let Some(parent) = p.parent() {
            create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut f = BufWriter::new(File::create(&p).map_err(|e| e.to_string())?);
        if let Some(encrypt_key) = &self.encrypt_key {
            let encrypted_data = encrypt(data.as_slice(), encrypt_key.as_str()).unwrap();
            f.write(encrypted_data.as_slice()).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            f.write(data.as_slice()).map_err(|e| e.to_string())?;
            Ok(())
        }
    }

    fn generate_file_path(&self, path: &str) -> PathBuf {
        let mut fullpath = self.path.clone();
        for p in path.split(DIR_SEPARATOR) {
            fullpath.push(p);
        }
        fullpath
    }

}

