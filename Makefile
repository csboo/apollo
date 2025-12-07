APOLLO_STATE_PATH?=apollo-state.cbor.encrypted
APOLLO_EVENT_TITLE?=hack-a-polo
APOLLO_MESTER_JELSZO?=Password

dx-args:=@server --server --features server_state_save @client --web

serve:
	APOLLO_STATE_PATH=${APOLLO_STATE_PATH} APOLLO_EVENT_TITLE=${APOLLO_EVENT_TITLE} APOLLO_MESTER_JELSZO=${APOLLO_MESTER_JELSZO} dx serve ${dx-args}

build:
	dx bundle ${dx-args}

bundle:
	dx bundle --release ${dx-args}

clean:
	cargo clean

help list:
	@echo *serve*: build, run and reload on changes
	@echo build: build in debug mode
	@echo bundle: build in release mode
	@echo clean: clean target
