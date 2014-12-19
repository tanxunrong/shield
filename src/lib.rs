
#![feature(unsafe_destructor)]
#[allow(dead_code)]

extern crate libc;

pub use state::{Mrb};
pub use class::{Class};

mod mruby;
mod class;
mod state;
mod value;
