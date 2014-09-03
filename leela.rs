
extern crate libc;
mod mruby
{

    use libc::{c_char};

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

    pub struct Mrb{
        state:*mut mrb_state
    }

    pub struct Value {
        val :*mut mrb_value
    }

    pub struct Class {
        class :*mut RClass
    }

    pub fn open() -> Mrb {
        let s = unsafe { mrb_open() } ;
        Mrb{state:s}
    }

    impl Mrb {
        pub fn load_str(&self,s:&str) {
            let cs = s.to_c_str();
            unsafe {
                mrb_load_string(self.state,cs.as_ptr());
            }
        }

        pub fn close(&self) {
            unsafe {
                mrb_close(self.state);
            }
        }

        pub fn define_class(&self,name:&str,clz:&Class) -> Class {
            let cs = name.to_c_str();
            let c = unsafe { mrb_define_class(self.state,cs.as_ptr(),clz.class) } ;
            Class{class:c}
        }

        pub fn obj_class(&self) -> Class {
            let s = "object";
            let c = unsafe { mrb_class_get(self.state,s.to_c_str().as_ptr()) } ;
            Class{class:c}
        }
    }
}

fn main() {
    let mut m = mruby::open();
    let exec = "[1,2,3].each do |i| puts i+1 end";
    m.load_str(exec);
    let o = &m.obj_class();
    m.load_str(exec);
    let c = m.define_class("Hello",o);
    m.close();
}
