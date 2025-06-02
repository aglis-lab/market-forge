tree:
	cargo modules structure --lib

build:
	cargo build --release

test_release_%:
	cargo test --release --test $*_test -- --nocapture

test_%:
	cargo test --test $*_test -- --nocapture

flame_%:
	cargo flamegraph -b $* -r -v -o ./temp/$*.svg
