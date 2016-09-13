.PHONY: deploy build

deploy: build
	apex deploy

build:
	cargo update --manifest-path functions/hello/Cargo.toml
