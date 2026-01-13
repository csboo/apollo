APOLLO_EVENT_TITLE?=hack-a-polo

dx-args:=@server --server --features server_state_save @client --web

serve:
	APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} dx serve ${dx-args}

serve-no-state:
	APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} dx serve --web

build:
	dx bundle ${dx-args}

bundle:
	dx bundle --release ${dx-args}

clean:
	cargo clean

help list:
	@echo "*serve*: build, run and reload on changes"
	@echo "serve-no-state: build, run and reload on changes, don't save server state"
	@echo "build: build in debug mode"
	@echo "bundle: build in release mode"
	@echo "clean: clean target"
