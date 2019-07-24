use std::path::PathBuf;
use rusqlite::{ NO_PARAMS, Connection };
use base64::{ encode, decode };
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use ::resource::material::storage::Storage;
use ::util::unwrap;

const TABLE_NAME: &str = "storage";

pub struct SQLiteStorage {
    name: String,
    con: Connection,
    key: Vec<u8>
}

impl SQLiteStorage {

    pub fn new(name: &str, source_path: PathBuf, base64_key: &str) -> Self {
        Self {
            name: name.to_owned(),
            con: Self::connect(source_path),
            key: unwrap(decode(base64_key))
        }
    }

    fn connect(source_path: PathBuf) -> Connection {
        Self::create_db_file_if_not_exists(&source_path);
        unwrap(Connection::open(source_path))
    }

    fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
        let mut encryptor = aes::cbc_encryptor(
                aes::KeySize::KeySize256,
                key,
                iv,
                blockmodes::PkcsPadding);
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        Ok(final_result)
    }

    fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let mut decryptor = aes::cbc_decryptor(
                aes::KeySize::KeySize256,
                key,
                iv,
                blockmodes::PkcsPadding);
    
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    
        loop {
            let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).map_err(|_| "decript failed"));
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
    
        Ok(final_result)
    }

    fn create_db_file_if_not_exists(path: &PathBuf) {
        if path.exists() { return; }
        let conn = unwrap(Connection::open(path));
        unwrap(conn.execute(&(format!("create table {} (
                      id     INTEGER PRIMARY KEY,
                      path   TEXT NOT NULL,
                      data   TEXT NOT NULL
                      )", &TABLE_NAME)), NO_PARAMS));
        unwrap(conn.execute(&(format!("create unique index uindex_path on {}(path)", &TABLE_NAME)), NO_PARAMS));
        conn.close().unwrap();
    }

}

impl Storage for SQLiteStorage {

    fn name(&self) -> String {
        self.name.clone()
    }


    fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let query_result: Result<String, _> = self.con.query_row("select data from storage where path = ?1", &[&path], |r| r.get(0));
        match query_result {
            Ok(val) => {
                let data = decode(&val).map_err(|e| e.to_string())?;
                Ok(Self::decrypt(data.as_slice(), &self.key[0 .. 32], &self.key[32 .. 48])?)
            },
            Err(_) => { Err(format!("Failed to read the file: {}", &path)) }
        }
    }

    fn list(&self, dir: Option<&str>) -> Result<Vec<String>, String> {
        let mut files: Vec<String> = Vec::new();
        match dir {
            None => {
                let mut stmt = unwrap(self.con.prepare(&format!("select path from {}", &TABLE_NAME)));
                let path_iter = unwrap(stmt.query_map(NO_PARAMS, |row| row.get(0)));
                for path in path_iter { files.push(path.unwrap()); }
            },
            Some(d) => {
                let mut stmt = unwrap(self.con.prepare(&format!("select path from {} where path like ?1", &TABLE_NAME)));
                let path_iter = unwrap(stmt.query_map(&[&(format!("{}%", d))], |row| row.get(0)));
                for path in path_iter { files.push(path.unwrap()); }
            }
        }
        Ok(files)
    }

    fn save(&self, path: &str, data: &Vec<u8>) -> Result<(), String> {
        let encrypted_data = Self::encrypt(data.as_slice(), &self.key[0 .. 32], &self.key[32 .. 48]).map_err(|_| "encrypt failed".to_owned())?;
        self.con.execute(
            &(format!("insert into {} (path, data) values (?1, ?2)", &TABLE_NAME)),
            &[&path, encode(&encrypted_data).as_str()]
        ).map(|_| ()).map_err(|_| "save failed".to_owned())
    }

}
