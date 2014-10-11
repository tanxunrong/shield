
use libc::{c_char};
use std::rc::Rc;

#[repr(C)]
    struct mrb_state;
#[repr(C)]
    struct mrb_value;
#[repr(C)]
    struct RClass;

#[link(name="m")]
#[link(name="mruby")]
extern "C" {

    fn mrb_open() -> *mut mrb_state;
    fn mrb_load_string(state:*mut mrb_state,s:*const c_char) -> mrb_value;
    fn mrb_close(state:*mut mrb_state);
    fn mrb_class_get(state:*mut mrb_state,name:*const c_char) -> *mut RClass;
    fn mrb_define_class(state:*mut mrb_state,name:*const c_char,class:*mut RClass) -> *mut RClass;
}

enum MvalType {
    TypeFalse ,
    TypeFree,
    TypeTrue,
    TypeFixnum,
    TypeSymbol,
    TypeUndef,
    TypeFloat,
    TypeCptr,
    TypeObject,
    TypeClass,
    TypeModule,
    TypeIClass,
    TypeSClass,
    TypeProc,
    TypeArray,
    TypeHash,
    TypeString,
    TypeRange,
    TypeException,
    TypeFile,
    TypeEnv,
    TypeData,
    TypeFiber,
    TypeMaxDefine
}

pub struct Mrb{
    state:Rc<*mut mrb_state>
}
/*
pub struct Value {
    val :*mut mrb_value
}
*/
pub struct Class {
    state:Rc<*mut mrb_state>,
    class:*mut RClass
}

pub fn open() -> Mrb {
    let s = unsafe { mrb_open() } ;
    Mrb{state:Rc::new(s)}
}

impl Mrb {
    pub fn load_str(&self,s:&str) {
        let cs = s.to_c_str();
        unsafe {
            mrb_load_string(*self.state.deref(),cs.as_ptr());
        }
    }

    pub fn define_class(&self,name:&str,clz:&Class) -> Class {
        let cs = name.to_c_str();
        let c = unsafe { 
            mrb_define_class(*self.state.deref(),cs.as_ptr(),clz.class) 
        };
        Class{state:self.state.clone(),class:c}
    }

    pub fn obj_class(&self) -> Class {
        let s = "Object";
        let c = unsafe {
            mrb_class_get(*self.state.deref(),s.to_c_str().as_ptr())
        };
        Class{state:self.state.clone(),class:c}
    }
}
/*
impl Drop for Mrb {
    fn drop(&mut self) {
        drop(self.state);
        println!("drop mrb");
    }
}
*/

#[test]
fn test_open_class() {
    let m = open();
    let exec = "[1,2,3].each do |i| puts i+1 end";
    m.load_str(exec);

    let exec2 = "[1,2,3].each do |i| puts i+1 end";
    m.load_str(exec2);

    let o = &m.obj_class();
    let c = m.define_class("Hello",o);
}
