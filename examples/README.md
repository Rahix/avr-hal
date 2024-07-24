`avr-hal` examples
==================
The subdirectories here contain various examples which demonstrate how to write
firmware using `avr-hal`.  Please note that often examples for a different
board can be easily ported to other hardware, so if you can't find something
for your board, look for examples with other hardware as well.

All examples are ready to use if you have the respective board available.  Just
switch to the subdirectory and run an example via cargo.  For example:

```bash
cd examples/arduino-uno

# Build and run it on a connected board
cargo run --bin uno-blink
```

You need to install [`ravedude`](https://crates.io/crates/ravedude) with `cargo install ravedude` to make
this work.