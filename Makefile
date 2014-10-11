
all : libmruby

libmruby : 
	if [ ! -d "./mruby" ];then git clone --depth=1 https://github.com/mruby/mruby;fi
	cd ./mruby && export CFLAGS=$(CFLAGS)" -fPIC" && make
	cp ./mruby/build/host/lib/libmruby.a ${OUT_DIR}/

.PHONY : all 
