#!/bin/sh

./target/bindgen -l mruby -o mruby.rs \
	-match mruby.h \
	-match value.h \
	-match object.h\
	-match array.h \
	-match hash.h\
	-match string.h\
	-match variable.h \
	-match class.h \
	-match compile.h \
	-match error.h \
	-match irep.h \
	-match proc.h\
	-match mrb_throw.h\
	$1
