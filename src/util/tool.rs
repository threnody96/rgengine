use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read, Write, stdout};
use std::any::Any;
use std::ops::DerefMut;
use std::cell::RefCell;
use ::director::{ Director };
use ::application::{ Application, Context };
use ::util::{ FpsManager };
use ::node::{ SceneLike };
use sdl2::render::{ Canvas, TextureCreator };
use sdl2::video::{ Window, WindowContext};
use sdl2::event::{ Event };

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

pub fn load_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    let f = File::open(path).map_err(|_| format!("ファイルの読み込みに失敗しました: {}", path.to_str().unwrap()))?;
    let mut bytes = BufReader::new(f).bytes();
    let mut result: Vec<u8> = vec![];
    while let Some(Ok(b)) = bytes.next() { result.push(b); }
    Ok(result)
}

pub fn director<T, R>(callback: T) -> R where T: FnOnce(&Director) -> R {
    ::DIRECTOR.with(|d| {
        callback(d)
    })
}

pub(crate) fn context<T, R>(callback: T) -> R where T: FnOnce(&'static mut Context) -> R {
    unsafe {
        callback(::CONTEXT.as_mut().unwrap())
    }
}

fn initialize_context(application: Rc<dyn Application>) {
    unsafe {
        ::CONTEXT = Some(Context::new(application));
    }
}

pub fn run(application: Rc<dyn Application>) {
    director(|d| d.set_application(application.clone()));
    initialize_context(application.clone());
    let mut event_pump = context(|c| &mut c.event_pump);
    let mut fps_manager = FpsManager::new(application.fps());
    director(|d| {
        let scene = application.application_did_finish_launching();
        d.set_scene(scene);
        d.get_scene().update();
    });
    let mut prev_sleep_time: i64 = 0;
    loop {
        prev_sleep_time = fps_manager.run(
            prev_sleep_time,
            || {
                director(|d| d.clear_render_points());
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} => {
                            director(|d| d.set_continuing(false));
                        },
                        _ => {}
                    }
                }
            },
            || {
                let prev_scene = director(|d| d.get_scene());
                prev_scene.update();
                let next_scene = director(|d| d.get_scene());
                prev_scene.id() == next_scene.id()
            },
            || {
                director(|r| r.update_resolution_size());
                director(|d| d.get_scene()).render(None);
                director(|r| r.render_canvas());
            }
        );
        if !director(|d| d.is_continuing()) { break; }
        director(|d| d.set_current_fps(fps_manager.fps()));
    }
}
