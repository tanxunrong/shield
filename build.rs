extern crate gcc;

use std::io::fs::PathExtensions;
use std::default::Default;
use std::os::{getenv, setenv};

fn main() {
    let mut cflags = getenv("CFLAGS").unwrap_or("".to_string());
    cflags.push_str(" -fPIC -std=c99");
    setenv("CFLAGS", cflags);

    let mruby_root = getenv("MRUBY_ROOT").unwrap_or("/usr".to_string());
    let hdir = mruby_root.push_str("/include");

    gcc::compile_library("libmrb.a", &gcc::Config {
        include_directories: vec!(Path::new(hdir)), .. Default::default()
    }, &["mrb/lib.c"]);
}
