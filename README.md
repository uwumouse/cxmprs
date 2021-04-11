# cxmprs
Rust Program that uses Huffman Coding to compress files.

## Compression
> Currently compression doesn't works well. File `file.txt` compression is 21%, but it's very small percent for this algorithm and that type of data (`Lorem Ipsum`), where a lot of chars are repeated lots of times.

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