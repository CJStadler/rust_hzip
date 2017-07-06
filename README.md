# Rust Hzip

Implementing the Huffman coding algorithm to gain experience with Rust.

Inspired by a college CS assignment to implement it in Python: https://github.com/CJStadler/HZipper

While working on this I became aware of a [previous implementation in Rust](http://sireliah.com/niusy/rust_huffman/). I am attempting to avoid referencing that as much as possible until I have my own working version, then I will compare and make improvements.

## TODO


### General

- [x] Read command line arguments.
- [x] Read string from a file.
- [x] Write string to a file.
- [x] Read bytes from a file.
- [x] Write bytes to a file.
- [x] Write bits to a file.
- [x] Read bits from a file.

### Encoding

- [x] Build frequency table.
- [ ] Build the code tree from the input.
- [ ] Write the code tree to the output.
- [ ] Encode the message using the code tree.
- [ ] Write the encoded message to the output.

### Decoding

- [ ] Read the decoding tree from the file.
- [ ] Decode the message using the tree.
- [ ] Write the decoded message to the output file.
