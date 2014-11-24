
//! # mruby
//! mruby binding

extern crate libc;
mod mruby;

pub struct Mrb {
    mrb : *mut mruby::mrb_state,
}

pub fn new_mrb() -> Mrb {
    let mrb = unsafe {
        mruby::mrb_open()
    };
    Mrb{mrb:mrb}
}

impl Mrb {
    fn close(&self) {
        unsafe {
            mruby::mrb_close(self.mrb);
        }
        std::mem::drop(self);
    }
}

#[test]
fn test_mrb_open() {
    let m = new_mrb();
    m.close();
}
