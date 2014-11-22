
//! # mruby
//! mruby binding

extern crate libc;
mod mruby;

#[test]
fn test_mrb_open() {
    unsafe {
        let mrb : *mut mruby::mrb_state = mruby::mrb_open();
        mruby::mrb_close(mrb);
    }
}
