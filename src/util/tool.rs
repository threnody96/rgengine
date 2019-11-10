use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read, Write, stdout};
use ::application::{ Application, Context };
use ::node::scene::transition::{ TransitionStatus };
use ::util::{ FpsManager, director };
use base64::{ decode };
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ WriteBuffer, ReadBuffer, BufferResult };
use backtrace::Backtrace;
use chrono::{ Local };

pub const DIR_SEPARATOR: char = '/';

pub const ENCRYPT_KEY: &'static str = env!("RESOURCE_ENCRYPT_KEY");

#[derive(Eq, PartialEq)]
pub enum BuildMode {
    Release,
    Development
}

#[derive(Clone, Copy)]
pub struct NoOption {}

#[cfg(not(debug_assertions))]
pub fn build_mode() -> BuildMode { BuildMode::Release }

#[cfg(debug_assertions)]
pub fn build_mode() -> BuildMode { BuildMode::Development }

pub fn exe_dir() -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let path = exe_path.as_path().parent().unwrap();
    path.to_path_buf()
}

pub fn load_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let f = File::open(path).map_err(|_| format!("ファイルの読み込みに失敗しました: {}", path.to_str().unwrap()))?;
    let mut bytes = BufReader::new(f).bytes();
    let mut result: Vec<u8> = vec![];
    while let Some(Ok(b)) = bytes.next() { result.push(b); }
    Ok(result)
}

pub fn encrypt(data: &[u8], key: &str) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let decoded_key = decode(key).unwrap();
    if decoded_key.len() != 48 {
        panic!("encrypt key must be base64 encoded 48 bytes data.");
    }
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        &decoded_key[0 .. 32],
        &decoded_key[32 .. 48],
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

pub fn decrypt(encrypted_data: &[u8], key: &str) -> Result<Vec<u8>, String> {
    let decoded_key = decode(key).unwrap();
    if decoded_key.len() != 48 {
        panic!("encrypt key must be base64 encoded 48 bytes data.");
    }
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        &decoded_key[0 .. 32],
        &decoded_key[32 .. 48],
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

pub(crate) fn with_context<T, R>(callback: T) -> R where T: FnOnce(&'static mut Context) -> R {
    unsafe {
        callback(::CONTEXT.as_mut().unwrap())
    }
}

fn initialize_context(application: Rc<dyn Application>) {
    unsafe {
        ::CONTEXT = Some(Context::new(application));
    }
}

fn output_error_log(err: &str) {
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

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|p| {
        let b = Backtrace::new();
        let err_msg = if let Some(s) = p.payload().downcast_ref::<String>() {
            s.to_string()
        } else if let Some(s) = p.payload().downcast_ref::<&str>() {
            s.to_string()
        } else {
            "Unknown Error".to_owned()
        };
        output_error_log(format!("[Error Report ({})]\n\n{}\n{:?}", Local::now(), err_msg, b).as_str());
    }));
}

pub fn run(application: Rc<dyn Application>) {
    set_panic_hook();
    director::set_application(application.clone());
    initialize_context(application.clone());
    let event_pump = with_context(|c| &mut c.event_pump);
    let mut fps_manager = FpsManager::new(application.fps());
    director::replace_scene(application.application_did_finish_launching(), ::NoOption);
    director::get_scene().start_update();
    let mut prev_sleep_time: i64 = 0;
    while director::is_continuing() {
        let (scene, mut prev_scene, transition) = (
            director::get_scene(), director::get_prev_scene(), director::get_scene_transition()
        );
        prev_sleep_time = fps_manager.run(
            prev_sleep_time,
            || {
                director::update_input_state(event_pump);
            },
            || {
                scene.start_update();
                let next_scene = director::get_scene();
                if next_scene.id() != scene.id() { next_scene.start_update(); }
            },
            || {
                director::update_resolution_size();
                scene.start_render();
                if let Some(p) = prev_scene.clone() {
                    p.start_render();
                }
                let status = director::render_canvas(scene.clone(), prev_scene.clone(), transition.clone());
                if status == TransitionStatus::Finish && prev_scene.is_some() {
                    director::destroy_prev_scene();
                    prev_scene = None;
                }
            }
        );
        director::set_current_fps(fps_manager.fps());
        director::clean_se();
        if director::is_quit() {
            application.on_quit();
        }
    }
    sdl2::mixer::close_audio();
}

