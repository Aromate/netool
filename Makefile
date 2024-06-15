build:
	cargo build --release

install:
	mkdir ~/.local/bin || true
	sudo rm ~/.local/bin/netool
	sudo install -m4755 ./target/release/binary ~/.local/bin/netool
