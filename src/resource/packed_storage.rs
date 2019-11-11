use std::path::PathBuf;
use std::fs::{ create_dir_all };
use rusqlite::{ Connection };
use base64::{ decode, encode };
use ::util::{ exe_dir, decrypt, encrypt, DIR_SEPARATOR, ENCRYPT_KEY };

pub struct PackedStorage {
    con: Connection,
    encrypt_key: Option<String>
}

impl PackedStorage {

    pub fn new(path: PathBuf, encrypt_key: Option<String>) -> Self {
        Self::initialize_db(&path);
        Self {
            con: Connection::open(path).unwrap(),
            encrypt_key: encrypt_key
        }
    }

    pub fn new_resource() -> Self {
        Self::from_exe_dir("resource.dat", Some(ENCRYPT_KEY.to_owned()))
    }

    pub fn from_exe_dir(path: &str, encrypt_key: Option<String>) -> Self {
        let mut resource_dir = exe_dir();
        let paths: Vec<&str> = path.split(DIR_SEPARATOR).collect();
        for p in paths { resource_dir.push(p); }
        Self::new(resource_dir, encrypt_key)
    }

    pub fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let query_result: Result<String, _> = self.con.query_row("select data from storage where path = ?1", &[&path], |r| r.get(0));
        match query_result {
            Ok(val) => {
                let data = decode(&val).map_err(|e| e.to_string())?;
                if let Some(key) = &self.encrypt_key {
                    Ok(decrypt(data.as_slice(), key.as_str())?)
                } else {
                    Ok(data)
                }
            },
            Err(_) => { Err(format!("ファイルの読み込みに失敗しました: {}", &path)) }
        }
    }

    pub fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        let d = if let Some(encrypt_key) = &self.encrypt_key {
            let encrypted_data = encrypt(data.as_slice(), encrypt_key).unwrap();
            encode(&encrypted_data)
        } else {
            encode(data)
        };
        if let Ok(_) = self.load(path) {
            self.con.execute("delete from storage where path = ?1", &[&path]).unwrap();
        }
        self.con.execute("insert into storage (path, data) values (?1, ?2)", &[&path, d.as_str()]).unwrap();
        Ok(())
    }

    pub fn initialize_db(path: &PathBuf) {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).unwrap();
        }
        if path.exists() { return; }
        let conn = Connection::open(path).unwrap();
        conn.execute("create table storage (
                      id     INTEGER PRIMARY KEY,
                      path   TEXT NOT NULL,
                      data   BLOB
                      )", params!()).unwrap();
        conn.execute("create unique index uindex_path on storage(path)", params!()).unwrap();
        conn.close().unwrap();
    }

}

