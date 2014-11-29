
#![feature(unsafe_destructor)]
#[allow(dead_code)]

extern crate libc;
use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::{HashMap};
use mruby::{mrb_open,mrb_close,mrb_state,mrb_value,Struct_RClass};
mod mruby;

pub type MrbState = Rc<RefCell<*mut mrb_state>>;
pub type MrbClass = Rc<RefCell<*mut Struct_RClass>>;

pub struct Mrb {
    mrb : MrbState,
    clzs : RefCell<HashMap<String,MrbClass>>
}

pub struct Class {
    mrb : MrbState,
    clz : MrbClass
}

impl Mrb {

    pub fn new() -> Mrb {
        let mrb = unsafe {
            mrb_open()
        };
        let state = unsafe { *mrb };
        let clzs : RefCell<HashMap<String,MrbClass>> = RefCell::new(HashMap::new());
        let m = Mrb{ mrb:Rc::new(RefCell::new(mrb)),clzs:clzs };
        m.clzs.borrow_mut().insert(String::from_str("Object"),Rc::new(RefCell::new(state.object_class)));
        m.clzs.borrow_mut().insert(String::from_str("Class"),Rc::new(RefCell::new(state.class_class)));
        m.clzs.borrow_mut().insert(String::from_str("String"),Rc::new(RefCell::new(state.string_class)));
        m.clzs.borrow_mut().insert(String::from_str("Array"),Rc::new(RefCell::new(state.array_class)));
        m.clzs.borrow_mut().insert(String::from_str("Hash"),Rc::new(RefCell::new(state.hash_class)));
        m
    }

    pub fn close(&self) {
        if std::rc::is_unique(&self.mrb) {
            let state = self.get_mrb();
            unsafe {
                mrb_close(state);
            }
        }
    }

    #[inline(always)]
    fn get_mrb(&self) -> *mut mrb_state {
        *(self.mrb).borrow_mut()
    }

    pub fn get_class(&self,name:&str) -> Option<Class> {
        let mut h = self.clzs.borrow_mut();
        let clzname = String::from_str(name);
        if h.get(&clzname).is_some() {
                return Some(Class{mrb:self.mrb.clone(),clz:h.get(&clzname).unwrap().clone()})
        }
        let mclz = unsafe { 
            mruby::mrb_class_get(self.get_mrb(), 
                                 name.as_slice().as_ptr() as *const libc::c_char) 
        };
        if mclz.is_null() {
            None
        } else {
            let class = Class{mrb:self.mrb.clone(),clz:Rc::new(RefCell::new(mclz))};
            h.insert(clzname,class.clz.clone());
            Some(class)
        }
    }

    pub fn load(&self,code:&str) -> mrb_value {
        unsafe {
            mruby::mrb_load_string(self.get_mrb(),
            code.as_slice().as_ptr() as *const libc::c_char)
        }
    }

}
/*
#[unsafe_destructor]
impl Drop for RefCell<*mut mrb_state> {
    fn drop(&mut self) {
        let mrb = *self.borrow_mut();
        if !mrb.is_null() {
            println!("mrb pointer null");
            unsafe {
                mrb_close(mrb);
            }
        }
    }
}
*/
#[test]
fn test_mrb_open() {
    let m = Mrb::new();
    m.close();
}

#[test]
fn test_get_class() {
    let m = Mrb::new();
    let arr_clz = m.get_class("Array").unwrap();
    assert!(m.get_class("Class").is_some());
    assert!(m.get_class("Object").is_some());
    assert!(m.get_class("Hash").is_some());
    assert!(m.get_class("String").is_some());
    m.close();
    assert!(m.get_class("String").is_some());
    assert!(std::rc::strong_count(&arr_clz.mrb) == 2);
}

#[test]
fn test_load_str() {
    let m = Mrb::new();
    let mut v = m.load("1..3.each do |i| puts i end");
    assert!(v.is_nil());
    m.close();
}
