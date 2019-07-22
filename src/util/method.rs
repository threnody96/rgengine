use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn exe_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.as_path().parent().unwrap();
    path.to_path_buf()
}

pub fn unwrap<O, E>(result: Result<O, E>) -> O
where E: ToString
{
    match result {
        Ok(o) => { o },
        Err(_) => {
            panic!("爆発しました");
        }
    }
}

pub fn load_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let f = File::open(path).map_err(|_| format!("Failed to read the file: {}", path.to_str().unwrap()))?;
    let mut bytes = BufReader::new(f).bytes();
    let mut result: Vec<u8> = vec![];
    while let Some(Ok(b)) = bytes.next() { result.push(b); }
    Ok(result)
}