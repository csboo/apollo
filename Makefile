NATIVE_TARGET:=$(shell rustup show | grep 'Default host: ' | sed 's/Default host: //')
EVENT_TITLE?=hack-a-polo
TARGET?=${NATIVE_TARGET}
PROFILE?=debug
CARGO_HACK_FEAT_ARGS:=--feature-powerset \
	--mutually-exclusive-features desktop,mobile \
	--mutually-exclusive-features desktop,web \
	--mutually-exclusive-features web,mobile

target-arg := --target ${TARGET}
dx-server-args := --no-default-features --features server_state_save --server ${target-arg}
dx-client-args := --no-default-features --web
dx-args:=@server ${dx-server-args} @client ${dx-client-args}
dist_p := target/dist
dx_c_p := target/dx
bin_ext := $(if $(findstring windows,${TARGET}),.exe,)
SERVER_TARGET ?= x86_64-unknown-linux-gnu
server-target-arg := --target ${SERVER_TARGET}
server_bin_ext := $(if $(findstring windows,${SERVER_TARGET}),.exe,)
server_build_cmd := build

ifeq (${PROFILE},release)
profile-arg:=--release
endif

ifneq (${NATIVE_TARGET},${SERVER_TARGET})
$(info setting build-command to zigbuild, as we are not compiling for the native target)
server_build_cmd := zigbuild
endif

default: serve

prepare-assets:
	# `asset!()` needs this...
	test -e assets/tailwind.css || touch assets/tailwind.css 

serve:
	APOLLO_EVENT_TITLE=${EVENT_TITLE} dx serve ${dx-args}

serve-no-state:
	APOLLO_EVENT_TITLE=${EVENT_TITLE} dx serve --web

check: prepare-assets
	cargo check --all-targets
	cargo check -F server_state_save
	cargo check -F web --target wasm32-unknown-unknown

check-all: prepare-assets
	cargo hack check ${CARGO_HACK_FEAT_ARGS} --no-dev-deps ${target-arg} --verbose

check-ci: 
	actionlint -verbose .github/workflows/**.yml

clippy: prepare-assets
	cargo clippy --no-deps

clippy-all: prepare-assets
	cargo hack clippy ${CARGO_HACK_FEAT_ARGS} ${target-arg} --all-targets --no-deps --verbose

format fmt:
	dx fmt
	cargo fmt

fmt-check format-check:
	cargo fmt --all --check --verbose
	dx fmt --check --verbose || echo "dx fmt isn't quite stable yet, don't worry much about it"

strict-check: check check-all clippy clippy-all fmt-check

bundle: prepare-assets
	rm -r ${dist_p} ${dx_c_p} || echo "deleting cache, would bloat otherwise"
	dx bundle --debug-symbols=false --verbose --out-dir ${dist_p} ${profile-arg} ${dx-args}

	# rename to include optional extension
	mv ${dist_p}/apollo ${dist_p}/apollo${bin_ext} || echo "same file, not moving"
	# rename to include platform
	tar -C ${dist_p} -cvf ${dist_p}/apollo-web-${TARGET}.tar apollo${bin_ext} public/
	tar -tf ${dist_p}/apollo-web-${TARGET}.tar

server-build:
	@echo 'probably `ulimit -n 1024`' # needed on my mac for sure
	cargo ${server_build_cmd} --release ${server-target-arg} --no-default-features --features server_state_save,web
	cp target/${SERVER_TARGET}/release/apollo${server_bin_ext} apollo-${SERVER_TARGET}${server_bin_ext}

clean:
	cargo clean

help list:
	@echo "*serve* [TARGET=]: build, run and reload on changes"
	@echo "serve-no-state [TARGET=]: build, run and reload on changes, don't persist server state"
	@echo "check: quick check whether code would compile"
	@echo "check-all [TARGET=]: same, with all reasonable feature-combinations"
	@echo "check-ci: validate github actions"
	@echo "clippy: quick clippy checks"
	@echo "clippy-all: same, with all reasonable feature-combinations"
	@echo "format|fmt: format code (rsx macros as well)"
	@echo "fmt-check: verify formatting (hopefully without modifying files)"
	@echo "strict-check: all checks from above"
	@echo "bundle [TARGET=, PROFILE=debug|release]: bundle apollo in web mode"
	@echo "server-build [SERVER_TARGET=]: build server binary in release mode"
	@echo "clean: clean target"
