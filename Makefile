clean:
	rm -rf ./build
build:
	CARGO_TARGET_DIR=./build_tmp cargo b --release && mkdir build; cp ./build_tmp/release/cxmprs ./build && rm -rf build_tmp