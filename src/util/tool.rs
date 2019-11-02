use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read, Write, stdout};
use ::director::{ Director };
use ::application::{ Application, Context };
use ::util::{ FpsManager };
use sdl2::event::{ Event };
use backtrace::Backtrace;

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
        output_error_log(format!("{}\n\n{:?}", err_msg, b).as_str());
    }));
}

pub fn run(application: Rc<dyn Application>) {
    set_panic_hook();
    director(|d| d.set_application(application.clone()));
    initialize_context(application.clone());
    let event_pump = context(|c| &mut c.event_pump);
    let mut fps_manager = FpsManager::new(application.fps());
    director(|d| {
        let scene = application.application_did_finish_launching();
        d.set_scene(scene);
        d.get_scene().start_update();
    });
    let mut prev_sleep_time: i64 = 0;
    while director(|d| d.is_continuing()) {
        prev_sleep_time = fps_manager.run(
            prev_sleep_time,
            || {
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
                prev_scene.start_update();
                let next_scene = director(|d| d.get_scene());
                prev_scene.id() == next_scene.id()
            },
            || {
                director(|d| d.update_resolution_size());
                director(|d| d.get_scene()).start_render();
                director(|d| d.render_canvas());
            }
        );
        director(|d| d.set_current_fps(fps_manager.fps()));
    }
}
