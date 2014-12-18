
#![feature(unsafe_destructor)]
#[allow(dead_code)]

extern crate libc;
use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::{HashMap};
use mruby::{mrb_open,mrb_close,mrb_state,mrb_value,Struct_RClass,mrb_int,Struct_mrbc_context};
mod mruby;

pub type MrbState = Rc<RefCell<*mut mrb_state>>;
pub type MrbClass = Rc<RefCell<*mut Struct_RClass>>;

pub struct Mrb {
    mrb : MrbState,
    clzs : RefCell<HashMap<String,MrbClass>>
}

#[deriving(PartialEq,Clone)]
pub struct Class {
    mrb : MrbState,
    clz : MrbClass
}

pub struct Context {
    mrb : MrbState,
    ctx : *mut Struct_mrbc_context
}

/// Traits for objects that has access to mrb_state.
/// Inspired by rust-hl-lua.
pub trait HasState {
    fn get_state(&self) -> *mut mrb_state;
}

impl HasState for Mrb {
    #[inline(always)]
    fn get_state(&self) -> *mut mrb_state {
        *(self.mrb).borrow_mut()
    }
}

impl HasState for Context {
    #[inline(always)]
    fn get_state(&self) -> *mut mrb_state {
        *(self.mrb).borrow_mut()
    }
}

impl HasState for Class {
    #[inline(always)]
    fn get_state(&self) -> *mut mrb_state {
        *(self.mrb).borrow_mut()
    }
}

/// Traits for objects that has access to Struct_RClass.
/// Inspired by rust-hl-lua.
pub trait HasClass {
    fn get_class(&self) -> *mut Struct_RClass;
}

impl HasClass for Class {
    #[inline(always)]
    fn get_class(&self) -> *mut Struct_RClass {
        *(self.clz).borrow_mut()
    }
}

impl Mrb {

    pub fn new() -> Mrb {
        let mrb = unsafe {
            mrb_open()
        };
        let state = mrb.clone();
        let clzs : RefCell<HashMap<String,MrbClass>> = RefCell::new(HashMap::new());
        let m = Mrb{ mrb:Rc::new(RefCell::new(mrb)),clzs:clzs };
        m.clzs.borrow_mut().insert(String::from_str("Object"),Rc::new(RefCell::new(unsafe {(*state).object_class} )));
        m.clzs.borrow_mut().insert(String::from_str("Class"),Rc::new(RefCell::new(
                    unsafe {
                    (*state).class_class
                    })));
        m.clzs.borrow_mut().insert(String::from_str("String"),Rc::new(RefCell::new(
                    unsafe {
                    (*state).string_class
                    })));
        m.clzs.borrow_mut().insert(String::from_str("Array"),Rc::new(RefCell::new(
                    unsafe {
                    (*state).array_class
                    })));
        m.clzs.borrow_mut().insert(String::from_str("Hash"),Rc::new(RefCell::new(
                    unsafe {
                    (*state).hash_class
                    })));
        unsafe { std::mem::forget(state); } 
        m
    }

    pub fn close(&self) {
        if std::rc::is_unique(&self.mrb) {
            let state = self.get_state();
            unsafe {
                mrb_close(state);
            }
        }
    }

    pub fn get_class(&self,name:&str) -> Option<Class> {
        let mut h = self.clzs.borrow_mut();
        let clzname = String::from_str(name);
        if h.get(&clzname).is_some() {
            return Some(Class{mrb:self.mrb.clone(),clz:h.get(&clzname).unwrap().clone()});
        }
        let state = self.get_state().clone() ;
        let defined = unsafe { 
            mruby::mrb_const_defined(self.get_state(),
                                                        mruby::wrap_mrb_obj_value((*state).object_class as *mut libc::c_void),
                                                        mruby::mrb_intern_cstr(self.get_state(),
                                                        name.as_ptr() as *const libc::c_char)) };
        let mclz = match defined {
            0 => unsafe {
                mruby::mrb_define_class(self.get_state(),
                                                  name.as_ptr() as *const libc::c_char,
                                                  (*state).object_class)
            },
            _ => unsafe { 
                mruby::mrb_class_get(self.get_state(), 
                                     name.as_ptr() as *const libc::c_char) 
            }
        };

        unsafe { std::mem::forget(state); }
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
            let state = self.get_state();
            mruby::mrb_load_string(state,code.as_ptr() as *const libc::c_char)

                /*
                   let state = self.get_state();
                   let sjmp = (*state).jmp;
                   if sjmp as uint != 0 {
                   panic!("jmp not null");
                   }
                   let jmp = libc::malloc(std::mem::size_of::<mruby::jmp_buf>() as libc::size_t) as *mut mruby::jmp_buf;
                   if mruby::setjmp(*jmp) as int == 0 {
                   (*state).jmp = jmp;
                   let val = mruby::mrb_load_string(state,code.as_ptr() as *const libc::c_char);
                   (*state).jmp = 0 as *mut _;
                   return val;
                   } else {
                   mruby::mrb_print_error(state);
                   panic!("fail to setjmp");
                   }
                   libc::free(jmp as *mut _);
                   */
        }
    }

    pub fn load_str_ctx(&self,code:&str,ctx:&Context) -> mrb_value {
        unsafe {
            let state = self.get_state();
            mruby::mrb_load_string_cxt(state,
                                       code.as_ptr() as *const libc::c_char,
                                       ctx.ctx)
        }
    }

    pub fn def_class(&self,name:&str,outer:&str) -> Option<Class> {

        if self.clzs.borrow().get(&String::from_str(name)).is_some() {
            panic!("class defined before");
        }

        self.get_class(name)

    }

    pub fn def_mod(&self,name:&str) -> Option<Class> {

        if self.clzs.borrow().get(&String::from_str(name)).is_some() {
            panic!("module defined before");
        }

        let clz = unsafe {
            mruby::mrb_define_module(self.get_state(),
            name.as_ptr() as *const libc::c_char
            )
        };
        let class = Class{mrb:self.mrb.clone(),clz:Rc::new(RefCell::new(clz))};
        let mut h = self.clzs.borrow_mut();
        h.insert(String::from_str(name),class.clz.clone());
        Some(class)

    }

    /*
    pub fn call(&self,v:&mrb_value,method:&str) {
        unsafe {
            mruby::mrb_funcall(self.get_state(),*v,
            method.as_ptr() as *const libc::c_char,0 as mrb_int);
        }
    }

    pub fn inspect(&self,v:&mrb_value) {
        self.call(v,"to_s");
    }
    */

    pub fn new_context(&self) -> Context {
        let ctx = unsafe { mruby::mrbc_context_new(self.get_state()) };
        Context { mrb:self.mrb.clone(),ctx:ctx }
    }

}

impl Class {
    fn new(&self) -> mrb_value {
        unsafe {
            mruby::mrb_obj_new(self.get_state(),self.get_class(),0 as mrb_int,0 as *const _)
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
fn test_obj_new() {
    let m = Mrb::new();
    let arr_clz = m.get_class("Array").unwrap();
    let v = arr_clz.new();
    assert!(!v.is_nil());
    m.close();
}

#[test]
fn test_load_str() {
    let m = Mrb::new();
    let mut v = m.load("1..3.each do |i| puts i end");
    assert!(v.is_nil());
    m.close();
}

#[test]
fn test_def_class() {
    let m = Mrb::new();
    let hello = m.def_class("Hello","Object");
    assert!(hello.is_some());
    m.close();
}

#[test]
fn test_def_mod() {
    let m = Mrb::new();
    let hello = m.def_mod("Hello");
    assert!(hello.is_some());
    m.close();
}
