tree:
	cargo modules structure --lib

test_%:
	cargo test --release --test $*_test -- --nocapture
