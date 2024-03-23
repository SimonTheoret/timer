install:
	cargo build --release
	echo "moving out of target"
	mv target/release/timer_rust .
	echo "changing name"
	mv timer_rust timer
	echo "build done"
