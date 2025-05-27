tree:
	cargo modules structure --lib

build:
	cargo build --release

test_release_%:
	cargo test --release --test $*_test -- --nocapture

test_%:
	cargo test --release --test $*_test -- --nocapture
