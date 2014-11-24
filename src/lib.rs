
//! # mruby
//! mruby binding

extern crate libc;
use mruby::{mrb_open,mrb_close,mrb_state,mrb_value};
mod mruby;

pub struct Mrb {
    mrb : *mut mrb_state,
}

pub fn new_mrb() -> Mrb {
    let mrb = unsafe {
        mrb_open()
    };
    Mrb{mrb:mrb}
}

impl Mrb {
    fn close(&self) {
        unsafe {
            mrb_close(self.mrb);
        }
        std::mem::drop(self);
    }

}

pub trait ToValue {
    fn to_mvalue(&self,m:Mrb) -> mrb_value;
}

impl ToValue for String {
    fn to_mvalue(&self,m:Mrb) -> mrb_value {
        unsafe {
            mruby::mrb_str_new(m.mrb,
                                self.as_slice().as_ptr() as *const libc::c_char,
                                self.len() as libc::size_t)
        }
    }
}

impl <V> ToValue for V where V:std::num::Int {
    fn to_mvalue(&self,m:Mrb) -> mrb_value {
        unsafe {

        }
    }
}

#[test]
fn test_mrb_open() {
    let m = new_mrb();
    m.close();
}
