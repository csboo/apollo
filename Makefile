APOLLO_EVENT_TITLE?=hack-a-polo

dx-args:=@server --server --features server_state_save @client --web

serve:
	APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} dx serve ${dx-args}

serve-no-state:
	APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} dx serve --web

build:
	dx bundle ${dx-args}

web-bundle:
	-rm -r web-apollo.zip target/dx # would bloat otherwise
	dx bundle --release ${dx-args}
	cp -r target/dx/apollo/release/web/public .
	-rm web-apollo.zip
	zip web-apollo public/* public/assets/*
	rm -r public
	unzip -l web-apollo.zip

server-build:
	@echo 'probably `ulimit -n 1024`' # needed on my mac for sure
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
