
use libc;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use state::{Mrb,HasState};
use mruby;
use mruby::{mrb_state,mrb_value};

pub type Pointer = Rc<RefCell<*mut libc::c_void>>;

#[deriving(Clone,PartialEq)]
pub enum Value {
    Nil,
    Undef,
    Int(i32),
    Float(f64),
    Symbol(u32),
    Cptr(Pointer),
    Bool(bool)
}

impl fmt::Show for Value {
    fn fmt(&self, ft : &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Cptr(ref ptr) => { ft.pad("Cptr") },
            &Value::Nil => { ft.pad("Nil") },
            &Value::Undef => { ft.pad("Undefined") },
            &Value::Int(i) => { ft.pad(format!("Fixnum({})",i).as_slice()) },
            &Value::Float(i) => { ft.pad(format!("Float({})",i).as_slice()) },
            &Value::Symbol(i) => { ft.pad(format!("Symbol({})",i).as_slice()) },
            &Value::Bool(i) => { ft.pad(format!("Bool({})",i).as_slice()) }
        }
    }
}

impl Value {

    //! convert value to mrb_value.Float & Cptr type need mrb_state.
    pub fn to_mrb(&self, mrb : Option<*mut mrb_state>) -> mrb_value {
        match self {
            &Value::Nil => {
                unsafe {
                    mruby::wrap_mrb_nil_value()
                }
            },
            &Value::Undef => {
                unsafe {
                    mruby::wrap_mrb_undef_value()
                }
            },
            &Value::Int(i) => {
                unsafe {
                    mruby::wrap_mrb_fixnum_value(i)
                }
            },
            &Value::Float(f) => {
                let m = mrb.unwrap();
                unsafe {
                    mruby::wrap_mrb_float_value(m,f)
                }
            },
            &Value::Symbol(u) => {
                unsafe {
                    mruby::wrap_mrb_symbol_value(u)
                }
            },
            //TODO unknown safety
            &Value::Cptr(ref p) =>  {
                let m = mrb.unwrap();
                unsafe {
                    mruby::wrap_mrb_cptr_value(m,*p.borrow_mut())
                }
            },
            &Value::Bool(b) => {
                match b {
                    true => { unsafe { mruby::wrap_mrb_true_value() } },
                    false => { unsafe { mruby::wrap_mrb_false_value() } }
                }
            }
        }
    }

}



#[test]
fn test_value() {
    let mut v1 = Value::Bool(true);
    assert!(v1.to_mrb(None).to_val() == v1);

    v1 = Value::Bool(false);
    assert!(v1.to_mrb(None).to_val() == v1);

    v1 = Value::Int(32);
    assert!(v1.to_mrb(None).to_val() == v1);

    v1 = Value::Symbol(239);
    assert!(v1.to_mrb(None).to_val() == v1);

    v1 = Value::Nil;
    assert!(v1.to_mrb(None).to_val() == v1);

    v1 = Value::Undef;
    assert!(v1.to_mrb(None).to_val() == v1);

    let m = Mrb::new();
    v1 = Value::Float(3.141592643);
    assert!(v1.to_mrb(Some(m.get_state())).to_val() == v1);

    m.close();
}
