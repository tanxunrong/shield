
use std::io::Command;
use std::io::fs::PathExtensions;

fn main() {
    let mruby = &Path::new("./libmruby.a");
    if !mruby.exists() {
        let git = Command::new("git")
            .arg(" clone https://github.com/mruby/mruby ")
            .status().unwrap();
        if !git.success() {
            panic!("fail to clone");
        }
        let build = Command::new("make").cwd(&Path::new("./mruby"))
            .env("CFLAGS","-fPIC -std=c99").status().unwrap();
        if !build.success() {
            panic!("fail to build");
        }
        let copy = Command::new("cp")
            .cwd(&Path::new("./mruby"))
            .arg("build/host/lib/libmruby.a")
            .arg("../")
            .status().unwrap();
        if !copy.success() {
            panic!("fail to cp");
        }
    }
}
