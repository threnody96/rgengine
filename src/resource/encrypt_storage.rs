use rusqlite::{ Connection };
use base64::{ decode };
use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{ WriteBuffer, ReadBuffer, BufferResult };
use ::util::{ exe_dir };

pub struct EncryptStorage {
    con: Connection,
    key: Vec<u8>
}

impl EncryptStorage {

    pub fn new() -> Self {
        let base64_key = env!("RESOURCE_ENCRYPT_KEY");
        let mut source_path = exe_dir();
        source_path.push("resource.dat");
        Self {
            con: Connection::open(source_path).unwrap(),
            key: decode(base64_key).unwrap()
        }
    }

    pub fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let query_result: Result<String, _> = self.con.query_row("select data from storage where path = ?1", &[&path], |r| r.get(0));
        match query_result {
            Ok(val) => {
                let data = decode(&val).map_err(|e| e.to_string())?;
                Ok(Self::decrypt(data.as_slice(), &self.key[0 .. 32], &self.key[32 .. 48])?)
            },
            Err(_) => { Err(format!("ファイルの読み込みに失敗しました: {}", &path)) }
        }
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
            let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).map_err(|_| "暗号化の解除に失敗しました"));
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }

        Ok(final_result)
    }

}

