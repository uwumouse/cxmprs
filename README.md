# cxmprs
Rust Program that uses Huffman Coding to compress files.


## Building
You need to have `cargo` on your system, then you run
```bash
make clean build
```
And you'll have your binary at `./cxmprs`

## Compression
Using:
```bash
# Will output to your_file.cxmp
./cxmprs your_file 
```
#### Try it
There's already built file in `/build`, so you can use it.
```bash
# Output will go to ./file.cxmp
./build/cxmprs ./file.txt
```

## Decompression
Feature in development...