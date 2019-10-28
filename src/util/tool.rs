use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::rc::Rc;
use std::io::{BufReader, Read, Write, stdout};
use std::any::Any;
use std::ops::DerefMut;
use std::cell::RefCell;
use ::director::{ Director, RenderDirector };
use ::application::{ Application };
use ::util::{ FpsManager };
use ::node::{ SceneLike };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };
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
    ::DIRECTOR.with(|d| {
        callback(d)
    })
}

pub fn canvas<T, R>(callback: T) -> R where T: FnOnce(&mut Canvas<Window>) -> R {
    render(|r| r.with_canvas(callback))
}

pub(crate) fn render<T, R>(callback: T) -> R where T: FnOnce(&'static mut RenderDirector<'static>) -> R {
    unsafe {
        if ::RENDER.is_none() { ::RENDER = Some(RenderDirector::new()); }
        callback(::RENDER.as_mut().unwrap())
    }
}

pub fn run(application: Rc<dyn Application>) {
    director(|d| d.set_application(application.clone()));
    let mut event_pump = render(|r| r.build(application.clone()));
    let mut fps_manager = FpsManager::new(application.fps());
    director(|d| {
        let scene = application.application_did_finish_launching();
        d.set_scene(scene);
        d.get_scene().update();
    });
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
        fps_manager.run(
            || {
                let prev_scene = director(|d| d.get_scene());
                prev_scene.update();
                let next_scene = director(|d| d.get_scene());
                prev_scene.id() == next_scene.id()
            },
            || {
                director(|d| d.get_scene()).render(None);
                render(|r| {
                    r.update_resolution_size(
                        application.resolution_size(),
                        application.resolution_policy()
                    );
                });
                render(|r| r.render_inner_canvas() );
                render(|r| r.render_canvas());
            }
        );
        director(|d| d.set_current_fps(fps_manager.fps()));
    }
}
