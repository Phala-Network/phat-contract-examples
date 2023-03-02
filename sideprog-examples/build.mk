PROG_NAME=$(shell toml get -r Cargo.toml package.name | tr '-' '_')
PROG=./target/wasm32-wasi/release/$(PROG_NAME).wasm

.PHONY: install clean always-rerun

install: $(PROG)
	mkdir -p $(PREFIX)
	cp $(PROG) $(PREFIX)/$(PROG_NAME).wasm

$(PROG): always-rerun
	cargo build --release --target wasm32-wasi

clean:
	cargo clean

always-rerun:
