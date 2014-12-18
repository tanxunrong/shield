extern crate gcc;

use std::io::fs::PathExtensions;
use std::default::Default;
use std::os::{getenv, setenv};

fn main() {
    let mut cflags = getenv("CFLAGS").unwrap_or("".to_string());
    cflags.push_str(" -fPIC -std=c99");
    setenv("CFLAGS", cflags);

    let mut mruby_root = getenv("MRUBY_ROOT").unwrap_or("/usr".to_string());
    mruby_root.push_str("/include");

    gcc::compile_library("libmrb.a", &gcc::Config {
        include_directories: vec!(Path::new(mruby_root)), .. Default::default()
    }, &["mrb/lib.c"]);
}
