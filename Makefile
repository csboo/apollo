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

ifeq (${PROFILE},release)
profile-arg:=--release
endif

default: serve

prepare-assets:
	# `asset!()` needs this...
	test -e assets/tailwind.css || touch assets/tailwind.css 

serve:
	APOLLO_EVENT_TITLE=${EVENT_TITLE} dx serve ${dx-args}

serve-no-state:
	APOLLO_EVENT_TITLE=${EVENT_TITLE} dx serve --web

check:
	cargo check --frozen --all-targets
	cargo check --frozen -F server_state_save
	cargo check --frozen -F web --target wasm32-unknown-unknown

check-all:
	cargo hack check --frozen ${CARGO_HACK_FEAT_ARGS} --no-dev-deps ${target-arg} --verbose

clippy: prepare-assets
	cargo clippy --frozen --no-deps

clippy-all: prepare-assets
	cargo hack clippy --frozen ${CARGO_HACK_FEAT_ARGS} ${target-arg} --all-targets --no-deps --verbose

format fmt:
	dx fmt --frozen
	cargo fmt --frozen

fmt-check format-check:
	cargo fmt --frozen --all --check --verbose
	dx fmt --frozen --check --verbose

strict-check: check check-all clippy clippy-all fmt-check

bundle: prepare-assets
	rm -r ${dist_p} ${dx_c_p} || echo "deleting cache, would bloat otherwise"
	dx bundle --frozen --debug-symbols=false --verbose --out-dir ${dist_p} ${profile-arg} ${dx-args}

	# rename to include optional extension
	mv ${dist_p}/apollo ${dist_p}/apollo${bin_ext} || echo "same file, not moving"
	# rename to include platform
	tar -cvf ${dist_p}/apollo-web-${TARGET}.tar ${dist_p}/apollo${bin_ext} ${dist_p}/public/
	tar -tf ${dist_p}/apollo-web-${TARGET}.tar

server-build:
	@echo 'probably `ulimit -n 1024`' # needed on my mac for sure
	target="x86_64-unknown-linux-gnu" # NOTE: feel free to overwrite this to musl
	build_cmd="build"
	if "${NATIVE_HOST}" -ne "${target}"; then; build_cmd="zigbuild"; fi
	cargo ${build_cmd} --frozen --release ${target-arg} --no-default-features --features server_state_save,web
	cp target/${target}/release/apollo apollo-x64-linux-gnu

clean:
	cargo clean

help list:
	@echo "*serve* [TARGET=]: build, run and reload on changes"
	@echo "serve-no-state [TARGET=]: build, run and reload on changes, don't persist server state"
	@echo "check: quick check whether code would compile"
	@echo "check-all [TARGET=]: same, with all reasonable feature-combinations"
	@echo "clippy: quick clippy checks"
	@echo "clippy-all: same, with all reasonable feature-combinations"
	@echo "format|fmt: format code (rsx macros as well)"
	@echo "fmt-check: verify formatting (hopefully without modifying files)"
	@echo "strict-check: all checks from above"
	@echo "bundle [TARGET=, PROFILE=debug|release]: bundle apollo for  artifact (TARGET=<triple> )"
	@echo "bundle: build in release mode"
	@echo "server-build: build for x64 linux gnu server"
	@echo "clean: clean target"
