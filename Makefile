#prog := #<build | debug | release | test | fmt | clean>
#@echo "usage: make $(prog)"

build:
	cargo build
debug:
	cargo build 
release:
	cargo build --release
test:
	cargo test
fmt:
	cargo fmt
clean:
	cargo clean
help:
	@echo "usage: "
	@echo "-- build: 	build default project."
	@echo "-- debug: 	build debug project."
	@echo "-- release: 	build release project."
	@echo "-- test: 	run tests."
	@echo "-- fmt:		run fmt."
	@echo "-- clean: 	clean project."