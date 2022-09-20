TARGET=wasm32-wasi
SIDE_WASM=sideprog/target/${TARGET}/release/sideprog.wasm

target/ink/start_sidevm.contract: sideprog.wasm
	cargo contract build --release

sideprog.wasm: ${SIDE_WASM}
	cp ${SIDE_WASM} .
	wasm-strip sideprog.wasm

.PHONY: ${SIDE_WASM}

${SIDE_WASM}:
	cargo build --manifest-path sideprog/Cargo.toml --release --target ${TARGET}

.PHONY: clean
clean:
	rm -rf sideprog.wasm sideprog.wasm.hash
	rm -rf target/ sideprog/target/
	rm -rf e2e/node_modules/

.PHONY: test
test: target/ink/start_sidevm.contract
	cd e2e && yarn && yarn test
