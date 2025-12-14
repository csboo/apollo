APOLLO_STATE_PATH?=apollo-state.cbor.encrypted
APOLLO_EVENT_TITLE?=hack-a-polo
APOLLO_MESTER_JELSZO?=Password

dx-args:=@server --server --features server_state_save @client --web

serve:
	APOLLO_STATE_PATH=${APOLLO_STATE_PATH} APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} APOLLO_MESTER_JELSZO=${APOLLO_MESTER_JELSZO} dx serve ${dx-args}

serve-no-state:
	APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} APOLLO_MESTER_JELSZO=${APOLLO_MESTER_JELSZO} dx serve --web

build:
	dx bundle ${dx-args}

web-bundle:
	dx bundle --release ${dx-args}
	cp -r target/dx/apollo/release/web/public .
	-rm web-apollo.zip
	zip web-apollo public/* public/assets/*
	rm -r public
	unzip -l web-apollo.zip

server-build:
	ulimit -n 1024 # needed on macos for sure
	cargo zigbuild --release --target x86_64-unknown-linux-gnu --no-default-features --features server_state_save,web
	cp target/x86_64-unknown-linux-gnu/release/apollo apollo-x86_64-linux-gnu

bundle: web-bundle server-build
	@echo "scp apollo-x86_64-linux-gnu web-apollo.zip <target>"

clean:
	cargo clean

help list:
	@echo "*serve*: build, run and reload on changes"
	@echo "serve-no-state: build, run and reload on changes, don't save server state"
	@echo "build: bundle in debug mode"
	@echo "web-bundle: bundle the web-client"
	@echo "server-build: build in release mode for x64-linux"
	@echo "bundle: web-bundle and server-build"
	@echo "clean: clean target"
