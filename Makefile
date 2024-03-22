install:
	cargo build --release
	mv ./target/release/timer_rust .
	mv timer_rust timer
