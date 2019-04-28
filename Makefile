all: build run

contrib_stt:
	pip3 install --upgrade pocketsphinx --user
	pip3 install --upgrade SpeechRecognition --user

stt:
	python3 scripts/stt.py&

mimic:
	git clone https://github.com/MycroftAI/mimic.git
	cd mimic
	./dependencies.sh --prefix="/usr/local"
	./autogen.sh
	./configure --prefix="/usr/local"
	make -j 8
	sudo make -j 8 install
	cd ..

dep: contrib_stt mimic

build:
	cargo build

run:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run

run_with_stt: stt
	RUST_BACKTRACE=1 RUST_LOG=info cargo run