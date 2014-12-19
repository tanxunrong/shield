
use mruby;
use state;
use state::{HasState,MrbState,MrbClass};
use mruby::{mrb_state,mrb_value,Struct_RClass,mrb_int};

#[deriving(PartialEq,Clone)]
pub struct Class {
    pub mrb : MrbState,
    pub clz : MrbClass
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

impl Class {
    pub fn new(&self) -> mrb_value {
        unsafe {
            mruby::mrb_obj_new(self.get_state(),self.get_class(),0 as mrb_int,0 as *const _)
        }
    }
}
