
extern crate libc;
mod mruby
{

use libc::{c_char};

#[repr(C)]
    struct mrb_state;
#[repr(C)]
    struct mrb_value;

#[link(name="m")]
#[link(name="mruby")]
    extern "C" {
        fn mrb_open() -> *mut mrb_state;
        fn mrb_load_string(state:*mut mrb_state,s:*const c_char) -> mrb_value;
        fn mrb_close(state:*mut mrb_state);
    }

    pub struct Mrb{
        state:*mut mrb_state
    }

    pub struct Value {
        val :*mut mrb_state
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
    }
}

fn main() {
    let mut m = mruby::open();
    let exec = "[1,2,3].each do |i| puts i+1 end";
    m.load_str(exec);
    m.close();
}
