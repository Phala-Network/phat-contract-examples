PREFIX ?= $(shell pwd)/dist
TARGET=wasm32-wasi

ifneq (, $(wildcard ./Cargo.toml))
CONTRACT_NAME=$(shell toml get -r Cargo.toml package.name | tr '-' '_')
SIDE_PROG_DIR=$(shell toml get -r Cargo.toml package.metadata.sideprog.path)
endif
ifneq (, $(SIDE_PROG_DIR))
SIDE_PROG=$(shell toml get -r $(SIDE_PROG_DIR)/Cargo.toml package.name | tr '-' '_')
SIDE_WASM=${SIDE_PROG_DIR}/target/${TARGET}/release/${SIDE_PROG}.wasm
endif

CONTRACT_OUTPUT=target/ink/${CONTRACT_NAME}.contract

all: ${CONTRACT_OUTPUT}

ifneq (, $(SIDE_PROG_DIR))
${CONTRACT_OUTPUT}: sideprog.wasm
endif

${CONTRACT_OUTPUT}: always-rerun
	cargo contract build --release

ifneq (, $(SIDE_PROG_DIR))
sideprog.wasm: ${SIDE_WASM}
	cp ${SIDE_WASM} ./sideprog.wasm
	wasm-strip sideprog.wasm

.PHONY: ${SIDE_WASM}

${SIDE_WASM}: always-rerun
	cargo build --manifest-path ${SIDE_PROG_DIR}/Cargo.toml --release --target ${TARGET}
endif

.PHONY: install clean always-rerun

ifneq (, $(CONTRACT_NAME))

install: ${CONTRACT_OUTPUT}
	mkdir -p ${PREFIX} 
	cp ${CONTRACT_OUTPUT} ${PREFIX}/${CONTRACT_NAME}.contract
ifneq (, $(SIDE_PROG_DIR))
	cp sideprog.wasm ${PREFIX}/${CONTRACT_NAME}.sidevm.wasm
endif
	
clean:
	rm -rf target/
ifneq (, $(SIDE_PROG_DIR))
	rm -rf sideprog.wasm sideprog.wasm.hash
	rm -rf ${SIDE_PROG_DIR}/target
endif

else
install:
	make install
clean:
	make clean
endif

always-rerun:
