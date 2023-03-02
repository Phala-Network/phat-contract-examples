 ifeq (, $(shell which toml))
 $(error "Command 'toml' is not found, consider install it with 'cargo install toml-cli'")
 endif

CONTRACTS := \
	sideprog-examples \
	start_sidevm \
	httpserver-hyper-tls \
	httpserver-routerify \
	httpserver-seamless \
	mqtt-broker \
	cross-contract \
	flip \
	hooks_test \
	http_client \
	logging \
	signing \
	unittests \
	use_cache \
	web3 \

PREFIX ?= $(shell pwd)/dist
export PREFIX

.PHONY: install always-rerun clean

install: $(PREFIX) $(CONTRACTS:%=install-%)

$(PREFIX):
	mkdir -p $(PREFIX)

install-%: always-rerun
	make install -C $* -f ../contract.mk

always-rerun:

clean: ${CONTRACTS:%=clean-%}

clean-%:
	make clean -C $* -f ../contract.mk

fmt: ${CONTRACTS:%=fmt-%}

fmt-%:
	make fmt -C $* -f ../contract.mk
