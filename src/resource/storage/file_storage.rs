use std::fs;
use std::path::PathBuf;
use std::fs::{ read_dir };
use std::io::{ BufWriter, Write };
use ::resource::storage::Storage;
use ::util::{ exe_dir, load_file, unwrap };

const SEPARATOR: char = '/';

pub struct FileStorage {
    name: String,
    dir: String,
}

impl FileStorage {

    pub fn new(name: String, dir: String) -> Self {
        let s = Self { name: name, dir: dir.clone() };
        if !s.generate_path(None).exists() { panic!(format!("dir not found: {}", dir)); }
        s
    }

    pub fn generate_path(&self, path: Option<&str>) -> PathBuf {
        let mut storage_dir = exe_dir();
        storage_dir.push(&self.dir);
        if path.is_none() { return storage_dir; }
        storage_dir.push(path.unwrap().to_owned());
        storage_dir
    }

    fn search(&self, current_dir: Option<&str>) -> Result<Vec<String>, String> {
        let mut result: Vec<String> = Vec::new();
        let dir_name = current_dir.clone().map(|d| format!("{}{}", d, SEPARATOR)).unwrap_or("".to_owned());
        let dir = self.generate_path(current_dir);
        for entry in unwrap(read_dir(dir)) {
            let path = unwrap(entry).path();
            let name = path.file_name().unwrap().to_str().unwrap();
            if path.is_file() {
                result.push(format!("{}{}", dir_name, name));
            } else {
                result.extend(unwrap(self.search(Some(format!("{}{}", dir_name, name).as_str()))));
            }
        }
        Ok(result)
    }

}

impl Storage for FileStorage {

    fn name(&self) -> String {
        self.name.clone()
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let pbuf = self.generate_path(Some(path));
        load_file(&pbuf)
    }

    fn list(&self, dir: Option<&str>) -> Result<Vec<String>, String> {
        self.search(dir)
    }

    fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        let real_path = self.generate_path(Some(path));
        let p = real_path.to_str().unwrap();
        let mut f = BufWriter::new(fs::File::create(p).unwrap());
        let r = f.write(data.as_slice());
        if r.is_ok() { Ok(()) } else { Err("Write failed".to_owned()) }
    }

}

