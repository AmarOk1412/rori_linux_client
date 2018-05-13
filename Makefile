all: build run

mimic:
	git clone https://github.com/MycroftAI/mimic.git
	cd mimic
	./dependencies.sh --prefix="/usr/local"
	./autogen.sh
	./configure --prefix="/usr/local"
	make -j 8
	sudo make -j 8 install
	cd ..

build:
	cargo build

run:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run
