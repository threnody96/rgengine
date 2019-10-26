use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read, Write, stdout};
use std::any::Any;
use ::director::{ Director };
use ::application::{ Application };
use ::node::{ SceneLike };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };

#[derive(Eq, PartialEq)]
pub enum BuildMode {
    Release,
    Development
}

#[cfg(not(debug_assertions))]
pub fn build_mode() -> BuildMode { BuildMode::Release }

#[cfg(debug_assertions)]
pub fn build_mode() -> BuildMode { BuildMode::Development }

pub fn exe_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.as_path().parent().unwrap();
    path.to_path_buf()
}

pub fn must<O, E>(result: Result<O, E>) -> O
    where E: ToString
{
    match result {
        Ok(o) => { return o; },
        Err(e) => {
            output_error_log(e);
            panic!("爆発しました");
        }
    }
}

fn output_error_log<E>(err: E)
    where E: ToString
{
    let mut file = match build_mode() {
        BuildMode::Release => {
            let mut dir = exe_dir();
            dir.push("application.log");
            Box::new(File::create(dir).unwrap()) as Box<dyn Write>
        },
        BuildMode::Development => { Box::new(stdout()) as Box<dyn Write> }
    };
    write!(file, "{}", err.to_string()).unwrap();
    file.flush().unwrap();
}

pub fn load_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let f = File::open(path).map_err(|_| format!("ファイルの読み込みに失敗しました: {}", path.to_str().unwrap()))?;
    let mut bytes = BufReader::new(f).bytes();
    let mut result: Vec<u8> = vec![];
    while let Some(Ok(b)) = bytes.next() { result.push(b); }
    Ok(result)
}

pub fn director<T, R>(callback: T) -> R where T: FnOnce(&Director) -> R {
    ::DIRECTOR.with(callback)
}

pub fn canvas<T, R>(callback: T) -> R where T: FnOnce(&mut Canvas<Window>) -> R {
    director(|d| d.with_canvas(callback))
}

pub fn run_with_scene(application: Rc<dyn Application>, scene: Rc<dyn SceneLike>) {
    director(|d| d.run_with_scene(application, scene));
}
