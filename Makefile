all: build run

contrib_stt:
	pip3 install --upgrade pocketsphinx --user
	pip3 install --upgrade SpeechRecognition --user
	sudo dnf install portaudio-devel -y
	pip3 install --upgrade PyAudio --user

stt:
	python3 scripts/stt.py&

mimic1:
	git clone https://github.com/MycroftAI/mimic.git
	cd mimic && \
	./dependencies.sh --prefix="/usr/local" && \
	./autogen.sh && \
	./configure --disable-lang-indic --prefix="/usr/local" && \
	make -j 8 && \
	sudo make -j 8 install
	touch .mimic1

dep: contrib_stt mimic1

build:
	cargo build

run:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run

run_with_stt: stt
	RUST_BACKTRACE=1 RUST_LOG=info cargo run
