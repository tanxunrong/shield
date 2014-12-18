shield
======

mruby binding in rust lang.It's in very alpha state.

build
-----
first,compile mruby with -fPIC flags and make sure *libmruby.a* is somewhere in *LD_LIBRARY_PATH*.
then,set *MRUBY_ROOT* env variable to the dir of mruby source code.
And *cargo test*.

