.PHONY: build-HelloWorldFunction
.PHONY: default

#
# Default "make" used for test compiles.
#
default:
	cargo build --target x86_64-unknown-linux-musl

#
# Not intended for use directly.  Use via "sam build"
#
build-HelloWorldFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)

