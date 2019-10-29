use std::fs::File;
use std::io::{ Write, stdout };
use ::util::{ build_mode, exe_dir, BuildMode };

pub fn output_error_log<E>(err: E)
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

pub trait Must {

    type Output;

    fn must(self) -> Self::Output;

    fn on_error<E>(&self) -> Box<FnOnce(E) -> Self::Output> where E: ToString {
        Box::new(|e: E| {
            output_error_log(e);
            panic!("Unknown Error");
        })
    }

}

impl <T> Must for Option<T> {

    type Output = T;

    fn must(self) -> T {
        let err = (&self).on_error::<String>();
        match self {
            Some(t) => { t },
            None => { err("".to_owned()) }
        }
    }

}

impl <O, E> Must for Result<O, E> where E: ToString {

    type Output = O;

    fn must(self) -> O {
        let err = (&self).on_error::<E>();
        match self {
            Ok(o) => { o },
            Err(e) => { err(e) }
        }
    }

}