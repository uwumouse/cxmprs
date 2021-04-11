clean:
	rm -rf ./build
build:
	CARGO_TARGET_DIR=./build cargo b --release && cp ./build/release/cxmprs ./ && rm -rf build