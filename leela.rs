
mod mruby
{

#[repr(C)]
    struct mrb_state;

#[link(name="m")]
#[link(name="mruby")]
    extern "C" {
        fn mrb_open() -> *mut mrb_state;
    }

    pub struct Mrb{
        state:*mut mrb_state
    }

    pub fn open() -> Mrb {
        let s = unsafe { mrb_open() } ;
        Mrb{state:s}
    }

    impl Mrb {
    }
}

fn main() {
    let m = mruby::open();
}
