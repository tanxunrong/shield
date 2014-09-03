
all : leela

leela : libmruby
	rustc leela.rs -L ./mruby/build/host/lib -o leela.out

libmruby : 
	if [ ! -d "./mruby" ];then git clone --depth=1 https://github.com/mruby/mruby;fi
	cd ./mruby && export CFLAGS=$(CFLAGS)" -fPIC" && make

clean : 
	rm leela.out

.PHONY : all leela clean
