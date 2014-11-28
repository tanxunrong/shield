
//! # mruby
//! mruby binding
#[allow(dead_code)]
extern crate libc;
use std::cell::{RefCell};
use mruby::{mrb_open,mrb_close,mrb_state,mrb_value,Struct_RClass};
mod mruby;

pub struct Mrb {
    mrb : RefCell<*mut mrb_state>
}

pub fn new_mrb() -> Mrb {
    let mrb = unsafe {
        mrb_open()
    };
    Mrb{mrb:RefCell::new(mrb)}
}

pub struct Class {
    clz : *mut Struct_RClass,
    outer : Option<*mut Struct_RClass>
}

impl Mrb {

    fn close(&self) {
        unsafe {
            mrb_close(*self.mrb.borrow_mut());
        }
        std::mem::drop(self);
    }

    fn get_class(&self,name:&str) -> Option<*mut Struct_RClass> {
        let mrb = unsafe { **self.mrb.borrow_mut() };
        let class = 
            match name {
                "object" => { mrb.object_class },
                "class" => { mrb.class_class },
                "string" => { mrb.string_class },
                "array" => { mrb.array_class },
                "hash" => { mrb.hash_class },
                _ => {
                    unsafe { mruby::mrb_class_get(*self.mrb.borrow_mut(),name.as_slice().as_ptr() as *const libc::c_char) }
                }
            };
        if class.is_null() {
            None
        } else {
            Some(class)
        }
    }

    fn load(&self,code:&str) -> mrb_value {
        unsafe {
            mruby::mrb_load_string(*self.mrb.borrow_mut(),code.as_slice().as_ptr() as *const libc::c_char)
        }
    }

}


#[test]
fn test_mrb_open() {
    let m = new_mrb();
    m.close();
}

#[test]
fn test_get_class() {
    let m = new_mrb();
    assert!(m.get_class("class").is_some())
    assert!(m.get_class("object").is_some())
    assert!(m.get_class("hash").is_some())
    assert!(m.get_class("array").is_some())
    assert!(m.get_class("string").is_some())
    m.close();
}

#[test]
fn test_load_str() {
    let m = new_mrb();
    let mut v = m.load("1..3.each do |i| puts i end");
    assert!(v.is_nil());
    m.close();
}
