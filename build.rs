extern crate gcc;

use std::io::Command;
use std::io::fs::PathExtensions;
use std::default::Default;
use std::os::{getenv, setenv};

fn main() {
    let mut cflags = getenv("CFLAGS").unwrap_or("".to_string());
    cflags.push_str(" -fPIC -std=c99");
    setenv("CFLAGS", cflags);

    let mruby_root = getenv("MRUBY_ROOT").unwrap();
    let mut mruby_incl = mruby_root.clone();
    mruby_incl.push_str("/include");

    gcc::compile_library("libmrb.a", &gcc::Config {
        include_directories: vec!(Path::new(mruby_incl)), .. Default::default()
    }, &["mrb/lib.c"]);

    let mut mruby_static = mruby_root.clone();
    mruby_static.push_str("/build/host/lib/libmruby.a");

    let mut make = Command::new("make").cwd(&Path::new(mruby_root)).env("CFLAGS","-fPIC -std=c99").status();
    match make {
        Ok(p) => {},
        Err(e) => { panic!("fail to make libmruby.a : {}",e); }
    }

    let mut out = getenv("OUT_DIR").unwrap_or("./".to_string());
    let mut cp = Command::new("cp").arg(mruby_static).arg(out).status();
    match cp {
        Ok(p) => {},
        Err(e) =>  { panic!("fail to cp libmruby.a : {}",e); }
    }
}
