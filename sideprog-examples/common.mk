all:
	cargo build --release --target wasm32-wasi

clean:
	rm -rf target/
