SIDE_PROG_DIR=mqtt-broker
SIDE_PROG=mqtt_v5_broker

SIDE_WASM=${SIDE_PROG_DIR}/target/wasm32-unknown-unknown/release/${SIDE_PROG}.wasm

target/ink/start_sidevm.contract: sideprog.wasm
	cargo +nightly contract build --release

sideprog.wasm: ${SIDE_WASM}
	cp ${SIDE_WASM} ./sideprog.wasm
	wasm-strip sideprog.wasm

.PHONY: ${SIDE_WASM}

${SIDE_WASM}:
	cargo build --manifest-path ${SIDE_PROG_DIR}/Cargo.toml --release --target wasm32-unknown-unknown

.PHONY: clean
clean:
	rm -rf sideprog.wasm
	rm -rf target/ ${SIDE_PROG_DIR}/target

