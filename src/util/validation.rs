use std::fs::File;
use std::io::{ Write, stdout };
use ::util::{ build_mode, exe_dir, BuildMode };

pub trait Validation {

    type Output;

    fn validate(self) -> Self::Output;

}

