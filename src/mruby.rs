/* automatically generated by rust-bindgen */
use libc::{int32_t,uint32_t,uint8_t,size_t,FILE,uint16_t};
use std::fmt;

pub type mrb_float = ::libc::c_double;
pub type mrb_int = int32_t;
pub type mrb_sym = ::libc::c_short;
pub type mrb_bool = uint8_t;
pub type Enum_mrb_vtype = ::libc::c_uint;
pub const MRB_TT_FALSE: ::libc::c_uint = 0;
pub const MRB_TT_FREE: ::libc::c_uint = 1;
pub const MRB_TT_TRUE: ::libc::c_uint = 2;
pub const MRB_TT_FIXNUM: ::libc::c_uint = 3;
pub const MRB_TT_SYMBOL: ::libc::c_uint = 4;
pub const MRB_TT_UNDEF: ::libc::c_uint = 5;
pub const MRB_TT_FLOAT: ::libc::c_uint = 6;
pub const MRB_TT_CPTR: ::libc::c_uint = 7;
pub const MRB_TT_OBJECT: ::libc::c_uint = 8;
pub const MRB_TT_CLASS: ::libc::c_uint = 9;
pub const MRB_TT_MODULE: ::libc::c_uint = 10;
pub const MRB_TT_ICLASS: ::libc::c_uint = 11;
pub const MRB_TT_SCLASS: ::libc::c_uint = 12;
pub const MRB_TT_PROC: ::libc::c_uint = 13;
pub const MRB_TT_ARRAY: ::libc::c_uint = 14;
pub const MRB_TT_HASH: ::libc::c_uint = 15;
pub const MRB_TT_STRING: ::libc::c_uint = 16;
pub const MRB_TT_RANGE: ::libc::c_uint = 17;
pub const MRB_TT_EXCEPTION: ::libc::c_uint = 18;
pub const MRB_TT_FILE: ::libc::c_uint = 19;
pub const MRB_TT_ENV: ::libc::c_uint = 20;
pub const MRB_TT_DATA: ::libc::c_uint = 21;
pub const MRB_TT_FIBER: ::libc::c_uint = 22;
pub const MRB_TT_MAXDEFINE: ::libc::c_uint = 23;
#[repr(C)]
pub struct Struct_mrb_value {
    pub value: Union_Unnamed1,
    pub tt: Enum_mrb_vtype,
}

#[cfg(target_arch="x86_64")]
#[repr(C)]
pub struct Union_Unnamed1 {
    pub data: [u64, ..1u],
}

#[cfg(target_arch="x86")]
#[repr(C)]
pub struct Union_Unnamed1 {
    pub data: [u32, ..2u],
}

impl Union_Unnamed1 {
    pub fn f(&mut self) -> *mut mrb_float {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn p(&mut self) -> *mut *mut ::libc::c_void {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn i(&mut self) -> *mut mrb_int {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn sym(&mut self) -> *mut mrb_sym {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn cf(& self) -> f64 {
        let f:mrb_float = unsafe { ::std::mem::transmute_copy(self) };
        f as f64
    }
    pub fn cp(& self) -> *const *const ::libc::c_void {
        unsafe { ::std::mem::transmute_copy(self) }
    }
    pub fn ci(& self) -> u32 {
        let i:mrb_int = unsafe { ::std::mem::transmute_copy(self) };
        i as u32
    }
    pub fn csym(& self) -> i16 {
        let sym:mrb_sym = unsafe { ::std::mem::transmute_copy(self) };
        sym as i16
    }
}
pub type mrb_value = Struct_mrb_value;
#[repr(C)]
pub struct Struct_RBasic {
    pub tt: Enum_mrb_vtype,
    pub color: uint32_t,
    pub flags: uint32_t,
    pub c: *mut Struct_RClass,
    pub gcnext: *mut Struct_RBasic,
}
#[repr(C)]
pub struct Struct_RClass;
#[repr(C)]
pub struct Struct_RObject {
    pub tt: Enum_mrb_vtype,
    pub color: uint32_t,
    pub flags: uint32_t,
    pub c: *mut Struct_RClass,
    pub gcnext: *mut Struct_RBasic,
    pub iv: *mut Struct_iv_tbl,
}
#[repr(C)]
pub struct Struct_iv_tbl;
#[repr(C)]
pub struct Struct_RFiber {
    pub tt: Enum_mrb_vtype,
    pub color: uint32_t,
    pub flags: uint32_t,
    pub c: *mut Struct_RClass,
    pub gcnext: *mut Struct_RBasic,
    pub cxt: *mut Struct_mrb_context,
}
pub type mrb_code = uint32_t;
pub type mrb_aspec = uint32_t;
#[repr(C)]
pub struct Struct_mrb_irep;

pub type mrb_allocf =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut Struct_mrb_state,
                               arg2: *mut ::libc::c_void, arg3: size_t,
                               arg4: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;
#[repr(C)]
pub struct Struct_Unnamed2 {
    pub mid: mrb_sym,
    pub _proc: *mut Struct_RProc,
    pub stackidx: ::libc::c_int,
    pub nregs: ::libc::c_int,
    pub argc: ::libc::c_int,
    pub pc: *mut mrb_code,
    pub err: *mut mrb_code,
    pub acc: ::libc::c_int,
    pub target_class: *mut Struct_RClass,
    pub ridx: ::libc::c_int,
    pub eidx: ::libc::c_int,
    pub env: *mut Struct_REnv,
}
#[repr(C)]
pub struct Struct_RProc;
#[repr(C)]
pub struct Struct_REnv;
pub type mrb_callinfo = Struct_Unnamed2;
pub type Enum_mrb_fiber_state = ::libc::c_uint;
pub const MRB_FIBER_CREATED: ::libc::c_uint = 0;
pub const MRB_FIBER_RUNNING: ::libc::c_uint = 1;
pub const MRB_FIBER_RESUMED: ::libc::c_uint = 2;
pub const MRB_FIBER_TERMINATED: ::libc::c_uint = 3;
#[repr(C)]
pub struct Struct_mrb_context {
    pub prev: *mut Struct_mrb_context,
    pub stack: *mut mrb_value,
    pub stbase: *mut mrb_value,
    pub stend: *mut mrb_value,
    pub ci: *mut mrb_callinfo,
    pub cibase: *mut mrb_callinfo,
    pub ciend: *mut mrb_callinfo,
    pub rescue: *mut *mut mrb_code,
    pub rsize: ::libc::c_int,
    pub ensure: *mut *mut Struct_RProc,
    pub esize: ::libc::c_int,
    pub status: Enum_mrb_fiber_state,
    pub fib: *mut Struct_RFiber,
}
pub type Enum_gc_state = ::libc::c_uint;
pub const GC_STATE_NONE: ::libc::c_uint = 0;
pub const GC_STATE_MARK: ::libc::c_uint = 1;
pub const GC_STATE_SWEEP: ::libc::c_uint = 2;
#[repr(C)]
pub struct Struct_mrb_state {
    pub jmp: *mut ::libc::c_void,
    pub allocf: mrb_allocf,
    pub c: *mut Struct_mrb_context,
    pub root_c: *mut Struct_mrb_context,
    pub exc: *mut Struct_RObject,
    pub globals: *mut Struct_iv_tbl,
    pub top_self: *mut Struct_RObject,
    pub object_class: *mut Struct_RClass,
    pub class_class: *mut Struct_RClass,
    pub module_class: *mut Struct_RClass,
    pub proc_class: *mut Struct_RClass,
    pub string_class: *mut Struct_RClass,
    pub array_class: *mut Struct_RClass,
    pub hash_class: *mut Struct_RClass,
    pub float_class: *mut Struct_RClass,
    pub fixnum_class: *mut Struct_RClass,
    pub true_class: *mut Struct_RClass,
    pub false_class: *mut Struct_RClass,
    pub nil_class: *mut Struct_RClass,
    pub symbol_class: *mut Struct_RClass,
    pub kernel_module: *mut Struct_RClass,
    pub heaps: *mut Struct_heap_page,
    pub sweeps: *mut Struct_heap_page,
    pub free_heaps: *mut Struct_heap_page,
    pub live: size_t,
    pub arena: *mut *mut Struct_RBasic,
    pub arena_capa: ::libc::c_int,
    pub arena_idx: ::libc::c_int,
    pub gc_state: Enum_gc_state,
    pub current_white_part: ::libc::c_int,
    pub gray_list: *mut Struct_RBasic,
    pub atomic_gray_list: *mut Struct_RBasic,
    pub gc_live_after_mark: size_t,
    pub gc_threshold: size_t,
    pub gc_interval_ratio: ::libc::c_int,
    pub gc_step_ratio: ::libc::c_int,
    pub gc_disabled: mrb_bool,
    pub gc_full: mrb_bool,
    pub is_generational_gc_mode: mrb_bool,
    pub out_of_memory: mrb_bool,
    pub majorgc_old_threshold: size_t,
    pub mems: *mut Struct_alloca_header,
    pub symidx: mrb_sym,
    pub name2sym: *mut Struct_kh_n2s,
    pub eException_class: *mut Struct_RClass,
    pub eStandardError_class: *mut Struct_RClass,
    pub ud: *mut ::libc::c_void,
}
#[repr(C)]
pub struct Struct_heap_page;
#[repr(C)]
pub struct Struct_alloca_header;
#[repr(C)]
pub struct Struct_kh_n2s;

pub type mrb_state = Struct_mrb_state;
pub type mrb_func_t =
    ::std::option::Option<extern "C" fn(arg1: *mut mrb_state, arg2: mrb_value)
                              -> mrb_value>;
pub type Enum_call_type = ::libc::c_uint;
pub const CALL_PUBLIC: ::libc::c_uint = 0;
pub const CALL_FCALL: ::libc::c_uint = 1;
pub const CALL_VCALL: ::libc::c_uint = 2;
pub const CALL_TYPE_MAX: ::libc::c_uint = 3;
pub type call_type = Enum_call_type;
#[repr(C)]
pub struct Struct_mrb_pool;

pub type mrb_pool = Struct_mrb_pool;

#[repr(C)]
pub struct Struct_mrbc_context {
    pub syms: *mut mrb_sym,
    pub slen: ::libc::c_int,
    pub filename: *mut ::libc::c_char,
    pub lineno: ::libc::c_short,
    pub partial_hook: ::std::option::Option<extern "C" fn
                                                (arg1:
                                                     *mut Struct_mrb_parser_state)
                                                -> ::libc::c_int>,
    pub partial_data: *mut ::libc::c_void,
    pub target_class: *mut Struct_RClass,
    pub capture_errors: mrb_bool,
    pub dump_result: mrb_bool,
    pub no_exec: mrb_bool,
}
pub type mrbc_context = Struct_mrbc_context;

#[repr(C)]
pub struct Struct_mrb_parser_state;

#[allow(dead_code)]
#[link(name = "mruby")]
extern "C" {
    pub fn mrb_define_class(arg1: *mut mrb_state, arg2: *const ::libc::c_char,
                            arg3: *mut Struct_RClass) -> *mut Struct_RClass;
    pub fn mrb_define_module(arg1: *mut mrb_state,
                             arg2: *const ::libc::c_char)
     -> *mut Struct_RClass;
    pub fn mrb_singleton_class(arg1: *mut mrb_state, arg2: mrb_value)
     -> mrb_value;
    pub fn mrb_include_module(arg1: *mut mrb_state, arg2: *mut Struct_RClass,
                              arg3: *mut Struct_RClass);
    pub fn mrb_define_method(arg1: *mut mrb_state, arg2: *mut Struct_RClass,
                             arg3: *const ::libc::c_char, arg4: mrb_func_t,
                             arg5: mrb_aspec);
    pub fn mrb_define_class_method(arg1: *mut mrb_state,
                                   arg2: *mut Struct_RClass,
                                   arg3: *const ::libc::c_char,
                                   arg4: mrb_func_t, arg5: mrb_aspec);
    pub fn mrb_define_singleton_method(arg1: *mut mrb_state,
                                       arg2: *mut Struct_RObject,
                                       arg3: *const ::libc::c_char,
                                       arg4: mrb_func_t, arg5: mrb_aspec);
    pub fn mrb_define_module_function(arg1: *mut mrb_state,
                                      arg2: *mut Struct_RClass,
                                      arg3: *const ::libc::c_char,
                                      arg4: mrb_func_t, arg5: mrb_aspec);
    pub fn mrb_define_const(arg1: *mut mrb_state, arg2: *mut Struct_RClass,
                            name: *const ::libc::c_char, arg3: mrb_value);
    pub fn mrb_undef_method(arg1: *mut mrb_state, arg2: *mut Struct_RClass,
                            arg3: *const ::libc::c_char);
    pub fn mrb_undef_class_method(arg1: *mut mrb_state,
                                  arg2: *mut Struct_RClass,
                                  arg3: *const ::libc::c_char);
    pub fn mrb_obj_new(mrb: *mut mrb_state, c: *mut Struct_RClass,
                       argc: ::libc::c_int, argv: *mut mrb_value)
     -> mrb_value;
    pub fn mrb_instance_new(mrb: *mut mrb_state, cv: mrb_value) -> mrb_value;
    pub fn mrb_class_new(mrb: *mut mrb_state, _super: *mut Struct_RClass)
     -> *mut Struct_RClass;
    pub fn mrb_module_new(mrb: *mut mrb_state) -> *mut Struct_RClass;
    pub fn mrb_class_defined(mrb: *mut mrb_state, name: *const ::libc::c_char)
     -> mrb_bool;
    pub fn mrb_class_get(mrb: *mut mrb_state, name: *const ::libc::c_char)
     -> *mut Struct_RClass;
    pub fn mrb_class_get_under(mrb: *mut mrb_state, outer: *mut Struct_RClass,
                               name: *const ::libc::c_char)
     -> *mut Struct_RClass;
    pub fn mrb_obj_dup(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    pub fn mrb_check_to_integer(mrb: *mut mrb_state, val: mrb_value,
                                method: *const ::libc::c_char) -> mrb_value;
    pub fn mrb_obj_respond_to(c: *mut Struct_RClass, mid: mrb_sym)
     -> mrb_bool;
    pub fn mrb_define_class_under(mrb: *mut mrb_state,
                                  outer: *mut Struct_RClass,
                                  name: *const ::libc::c_char,
                                  _super: *mut Struct_RClass)
     -> *mut Struct_RClass;
    pub fn mrb_define_module_under(mrb: *mut mrb_state,
                                   outer: *mut Struct_RClass,
                                   name: *const ::libc::c_char)
     -> *mut Struct_RClass;
    pub fn mrb_get_args(mrb: *mut mrb_state,
                        format: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn mrb_funcall(arg1: *mut mrb_state, arg2: mrb_value,
                       arg3: *const ::libc::c_char, arg4: ::libc::c_int, ...)
     -> mrb_value;
    pub fn mrb_funcall_argv(arg1: *mut mrb_state, arg2: mrb_value,
                            arg3: mrb_sym, arg4: ::libc::c_int,
                            arg5: *mut mrb_value) -> mrb_value;
    pub fn mrb_funcall_with_block(arg1: *mut mrb_state, arg2: mrb_value,
                                  arg3: mrb_sym, arg4: ::libc::c_int,
                                  arg5: *mut mrb_value, arg6: mrb_value)
     -> mrb_value;
    pub fn mrb_intern_cstr(arg1: *mut mrb_state, arg2: *const ::libc::c_char)
     -> mrb_sym;
    pub fn mrb_intern(arg1: *mut mrb_state, arg2: *const ::libc::c_char,
                      arg3: size_t) -> mrb_sym;
    pub fn mrb_intern_str(arg1: *mut mrb_state, arg2: mrb_value) -> mrb_sym;
    pub fn mrb_check_intern_cstr(arg1: *mut mrb_state,
                                 arg2: *const ::libc::c_char) -> mrb_value;
    pub fn mrb_check_intern(arg1: *mut mrb_state, arg2: *const ::libc::c_char,
                            arg3: size_t) -> mrb_value;
    pub fn mrb_check_intern_str(arg1: *mut mrb_state, arg2: mrb_value)
     -> mrb_value;
    pub fn mrb_sym2name(arg1: *mut mrb_state, arg2: mrb_sym)
     -> *const ::libc::c_char;
    pub fn mrb_sym2name_len(arg1: *mut mrb_state, arg2: mrb_sym,
                            arg3: *mut size_t) -> *const ::libc::c_char;
    pub fn mrb_sym2str(arg1: *mut mrb_state, arg2: mrb_sym) -> mrb_value;
    pub fn mrb_str_format(arg1: *mut mrb_state, arg2: ::libc::c_int,
                          arg3: *const mrb_value, arg4: mrb_value)
     -> mrb_value;
    pub fn mrb_malloc(arg1: *mut mrb_state, arg2: size_t)
     -> *mut ::libc::c_void;
    pub fn mrb_calloc(arg1: *mut mrb_state, arg2: size_t, arg3: size_t)
     -> *mut ::libc::c_void;
    pub fn mrb_realloc(arg1: *mut mrb_state, arg2: *mut ::libc::c_void,
                       arg3: size_t) -> *mut ::libc::c_void;
    pub fn mrb_realloc_simple(arg1: *mut mrb_state, arg2: *mut ::libc::c_void,
                              arg3: size_t) -> *mut ::libc::c_void;
    pub fn mrb_malloc_simple(arg1: *mut mrb_state, arg2: size_t)
     -> *mut ::libc::c_void;
    pub fn mrb_obj_alloc(arg1: *mut mrb_state, arg2: Enum_mrb_vtype,
                         arg3: *mut Struct_RClass) -> *mut Struct_RBasic;
    pub fn mrb_free(arg1: *mut mrb_state, arg2: *mut ::libc::c_void);
    pub fn mrb_str_new(mrb: *mut mrb_state, p: *const ::libc::c_char,
                       len: size_t) -> mrb_value;
    pub fn mrb_str_new_cstr(arg1: *mut mrb_state, arg2: *const ::libc::c_char)
     -> mrb_value;
    pub fn mrb_str_new_static(mrb: *mut mrb_state, p: *const ::libc::c_char,
                              len: size_t) -> mrb_value;
    pub fn mrb_open() -> *mut mrb_state;
    pub fn mrb_open_allocf(arg1: mrb_allocf, ud: *mut ::libc::c_void)
     -> *mut mrb_state;
    pub fn mrb_close(arg1: *mut mrb_state);
    pub fn mrb_top_self(arg1: *mut mrb_state) -> mrb_value;
    pub fn mrb_run(arg1: *mut mrb_state, arg2: *mut Struct_RProc,
                   arg3: mrb_value) -> mrb_value;
    pub fn mrb_context_run(arg1: *mut mrb_state, arg2: *mut Struct_RProc,
                           arg3: mrb_value, arg4: ::libc::c_uint)
     -> mrb_value;
    pub fn mrb_p(arg1: *mut mrb_state, arg2: mrb_value);
    pub fn mrb_obj_id(obj: mrb_value) -> mrb_int;
    pub fn mrb_obj_to_sym(mrb: *mut mrb_state, name: mrb_value) -> mrb_sym;
    pub fn mrb_obj_eq(arg1: *mut mrb_state, arg2: mrb_value, arg3: mrb_value)
     -> mrb_bool;
    pub fn mrb_obj_equal(arg1: *mut mrb_state, arg2: mrb_value,
                         arg3: mrb_value) -> mrb_bool;
    pub fn mrb_equal(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    pub fn mrb_Integer(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    pub fn mrb_Float(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    pub fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    pub fn mrb_eql(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    pub fn mrb_garbage_collect(arg1: *mut mrb_state);
    pub fn mrb_full_gc(arg1: *mut mrb_state);
    pub fn mrb_incremental_gc(arg1: *mut mrb_state);
    pub fn mrb_gc_arena_save(arg1: *mut mrb_state) -> ::libc::c_int;
    pub fn mrb_gc_arena_restore(arg1: *mut mrb_state, arg2: ::libc::c_int);
    pub fn mrb_gc_mark(arg1: *mut mrb_state, arg2: *mut Struct_RBasic);
    pub fn mrb_field_write_barrier(arg1: *mut mrb_state,
                                   arg2: *mut Struct_RBasic,
                                   arg3: *mut Struct_RBasic);
    pub fn mrb_write_barrier(arg1: *mut mrb_state, arg2: *mut Struct_RBasic);
    pub fn mrb_check_convert_type(mrb: *mut mrb_state, val: mrb_value,
                                  _type: Enum_mrb_vtype,
                                  tname: *const ::libc::c_char,
                                  method: *const ::libc::c_char) -> mrb_value;
    pub fn mrb_any_to_s(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    pub fn mrb_obj_classname(mrb: *mut mrb_state, obj: mrb_value)
     -> *const ::libc::c_char;
    pub fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value)
     -> *mut Struct_RClass;
    pub fn mrb_class_path(mrb: *mut mrb_state, c: *mut Struct_RClass)
     -> mrb_value;
    pub fn mrb_convert_type(mrb: *mut mrb_state, val: mrb_value,
                            _type: Enum_mrb_vtype,
                            tname: *const ::libc::c_char,
                            method: *const ::libc::c_char) -> mrb_value;
    pub fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value,
                              c: *mut Struct_RClass) -> mrb_bool;
    pub fn mrb_obj_inspect(mrb: *mut mrb_state, _self: mrb_value)
     -> mrb_value;
    pub fn mrb_obj_clone(mrb: *mut mrb_state, _self: mrb_value) -> mrb_value;
    pub fn mrb_exc_new(mrb: *mut mrb_state, c: *mut Struct_RClass,
                       ptr: *const ::libc::c_char, len: ::libc::c_long)
     -> mrb_value;
    pub fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value);
    pub fn mrb_raise(mrb: *mut mrb_state, c: *mut Struct_RClass,
                     msg: *const ::libc::c_char);
    pub fn mrb_raisef(mrb: *mut mrb_state, c: *mut Struct_RClass,
                      fmt: *const ::libc::c_char, ...);
    pub fn mrb_name_error(mrb: *mut mrb_state, id: mrb_sym,
                          fmt: *const ::libc::c_char, ...);
    pub fn mrb_warn(mrb: *mut mrb_state, fmt: *const ::libc::c_char, ...);
    pub fn mrb_bug(mrb: *mut mrb_state, fmt: *const ::libc::c_char, ...);
    pub fn mrb_print_backtrace(mrb: *mut mrb_state);
    pub fn mrb_print_error(mrb: *mut mrb_state);
    pub fn mrb_yield(mrb: *mut mrb_state, b: mrb_value, arg: mrb_value)
     -> mrb_value;
    pub fn mrb_yield_argv(mrb: *mut mrb_state, b: mrb_value,
                          argc: ::libc::c_int, argv: *mut mrb_value)
     -> mrb_value;
    pub fn mrb_gc_protect(mrb: *mut mrb_state, obj: mrb_value);
    pub fn mrb_to_int(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    pub fn mrb_check_type(mrb: *mut mrb_state, x: mrb_value,
                          t: Enum_mrb_vtype);
    pub fn mrb_define_alias(mrb: *mut mrb_state, klass: *mut Struct_RClass,
                            name1: *const ::libc::c_char,
                            name2: *const ::libc::c_char);
    pub fn mrb_class_name(mrb: *mut mrb_state, klass: *mut Struct_RClass)
     -> *const ::libc::c_char;
    pub fn mrb_define_global_const(mrb: *mut mrb_state,
                                   name: *const ::libc::c_char,
                                   val: mrb_value);
    pub fn mrb_block_proc() -> mrb_value;
    pub fn mrb_attr_get(mrb: *mut mrb_state, obj: mrb_value, id: mrb_sym)
     -> mrb_value;
    pub fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    pub fn mrb_obj_is_instance_of(mrb: *mut mrb_state, obj: mrb_value,
                                  c: *mut Struct_RClass) -> mrb_bool;
    pub fn mrb_pool_open(arg1: *mut mrb_state) -> *mut Struct_mrb_pool;
    pub fn mrb_pool_close(arg1: *mut Struct_mrb_pool);
    pub fn mrb_pool_alloc(arg1: *mut Struct_mrb_pool, arg2: size_t)
     -> *mut ::libc::c_void;
    pub fn mrb_pool_realloc(arg1: *mut Struct_mrb_pool,
                            arg2: *mut ::libc::c_void, oldlen: size_t,
                            newlen: size_t) -> *mut ::libc::c_void;
    pub fn mrb_pool_can_realloc(arg1: *mut Struct_mrb_pool,
                                arg2: *mut ::libc::c_void, arg3: size_t)
     -> mrb_bool;
    pub fn mrb_alloca(mrb: *mut mrb_state, arg1: size_t)
     -> *mut ::libc::c_void;

    pub fn mrb_parser_new(arg1: *mut mrb_state)
     -> *mut Struct_mrb_parser_state;
    pub fn mrb_parser_free(arg1: *mut Struct_mrb_parser_state);
    pub fn mrb_parser_parse(arg1: *mut Struct_mrb_parser_state,
                            arg2: *mut mrbc_context);
    pub fn mrb_parser_set_filename(arg1: *mut Struct_mrb_parser_state,
                                   arg2: *const ::libc::c_char);
    pub fn mrb_parser_get_filename(arg1: *mut Struct_mrb_parser_state,
                                   idx: uint16_t) -> *const ::libc::c_char;
    pub fn mrb_parse_file(arg1: *mut mrb_state, arg2: *mut FILE,
                          arg3: *mut mrbc_context)
     -> *mut Struct_mrb_parser_state;
    pub fn mrb_parse_string(arg1: *mut mrb_state, arg2: *const ::libc::c_char,
                            arg3: *mut mrbc_context)
     -> *mut Struct_mrb_parser_state;
    pub fn mrb_parse_nstring(arg1: *mut mrb_state,
                             arg2: *const ::libc::c_char, arg3: ::libc::c_int,
                             arg4: *mut mrbc_context)
     -> *mut Struct_mrb_parser_state;
    pub fn mrb_generate_code(arg1: *mut mrb_state,
                             arg2: *mut Struct_mrb_parser_state)
     -> *mut Struct_RProc;
    pub fn mrb_load_file(arg1: *mut mrb_state, arg2: *mut FILE) -> mrb_value;
    pub fn mrb_load_string(mrb: *mut mrb_state, s: *const ::libc::c_char)
     -> mrb_value;
    pub fn mrb_load_nstring(mrb: *mut mrb_state, s: *const ::libc::c_char,
                            len: ::libc::c_int) -> mrb_value;
    pub fn mrb_load_file_cxt(arg1: *mut mrb_state, arg2: *mut FILE,
                             cxt: *mut mrbc_context) -> mrb_value;
    pub fn mrb_load_string_cxt(mrb: *mut mrb_state, s: *const ::libc::c_char,
                               cxt: *mut mrbc_context) -> mrb_value;
    pub fn mrb_load_nstring_cxt(mrb: *mut mrb_state, s: *const ::libc::c_char,
                                len: ::libc::c_int, cxt: *mut mrbc_context)
     -> mrb_value;
}


impl mrb_value {

    pub fn is_nil(&self) -> bool {
        if self.tt == MRB_TT_FALSE && self.value.cf() != 0f64 {
            return true;
        }
        return false;
    }
}

impl fmt::Show for mrb_value {
    fn fmt(&self,ft:&mut fmt::Formatter) -> Result<(),fmt::Error> {
        if self.tt == MRB_TT_FIXNUM {
            let fixnum = self.value.ci();
            return fixnum.fmt(ft);
        }
        else if self.is_nil() {
            return ft.pad("nil");
        }
        else if self.tt == MRB_TT_FALSE {
            return ft.pad("false");
        }
        else if self.tt == MRB_TT_TRUE {
            return ft.pad("true");
        }
        else if self.tt == MRB_TT_SYMBOL {
            return self.value.csym().fmt(ft);
        }
        else if self.tt == MRB_TT_UNDEF {
            return ft.pad("undefined");
        }
        else if self.tt == MRB_TT_FLOAT {
            return self.value.cf().fmt(ft);
        }
        else if self.tt == MRB_TT_CPTR {
            return ft.pad("cptr");
        }
        else if self.tt == MRB_TT_PROC {
            return ft.pad("proc");
        }
        else if self.tt == MRB_TT_OBJECT {
            return ft.pad("obj");
        }
        else {
            return ft.pad("unknown");
        }
    }
}
