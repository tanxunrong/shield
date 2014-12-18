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
