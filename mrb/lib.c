#include <stdlib.h>
#include <stdio.h>
#include <setjmp.h>
#include <mruby.h>
#include <mruby/irep.h>
#include <mruby/error.h>
#include <mruby/debug.h>
#include <mruby/variable.h>

mrb_value wrap_mrb_obj_value(void *p)
{
    return mrb_obj_value(p);
}

mrb_value wrap_mrb_fixnum_value(mrb_int i)
{
    return mrb_fixnum_value(i);
}

mrb_value wrap_mrb_float_value(mrb_state *m,mrb_float f)
{
    return mrb_float_value(m,f);
}

mrb_value wrap_mrb_true_value()
{
    return mrb_true_value();
}

mrb_value wrap_mrb_false_value()
{
    return mrb_false_value();
}

mrb_value wrap_mrb_symbol_value(mrb_sym i)
{
    return mrb_symbol_value(i);
}

mrb_value wrap_mrb_nil_value()
{
    return mrb_nil_value();
}

mrb_value wrap_mrb_undef_value()
{
    return mrb_undef_value();
}

mrb_value wrap_mrb_cptr_value(mrb_state *m,void *p)
{
    return mrb_cptr_value(m,p);
}

void * wrap_mrb_ptr (mrb_value o) {    return       (o).value.p;}
void *wrap_mrb_cptr (mrb_value o) { return      wrap_mrb_ptr(o); }
mrb_float wrap_mrb_float (mrb_value o) { return     o.value.f; }
mrb_int wrap_mrb_fixnum (mrb_value o) { return    (o).value.i; }
mrb_sym wrap_mrb_symbol (mrb_value o) { return    (o).value.sym; }
int wrap_mrb_type (mrb_value o) { return      (o).tt; }

