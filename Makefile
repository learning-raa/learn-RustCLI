binname="learn-RustCLI"

help:
	@cat Makefile

pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

edit:
	@nvim ./src/main.rs

run: release size
	@./target/release/$(binname)
release:
	@cargo rustc --release -- -C prefer-dynamic
test:
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)
path:
	export LD_LIBRARY_PATH='/home/configurator/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib'

clean:
	@cargo clean
