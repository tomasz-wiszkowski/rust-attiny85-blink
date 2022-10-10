## Digispark ATtiny85 Led Flashing

Built as a scaffolding for future attiny85 rust prototypes of mine.

### Requirements

To use this, you need to 
* download, or better - *build* - `micronucleus` from [Github repo](https://github.com/micronucleus/micronucleus/tree/master). 
* Rust nightly 
* AVR GCC

```    
rustup install nightly
rustup default nightly
sudo apt install avr-gcc
```

### ATTiny85 notes

Due to limitations of the programmable memory on this microcontroller,
* be sure to build and run `---release` version (`cargo build --release` or `cargo run --release`) - otherwise your code just won't fit.
* the code interfaces registers directly to preserve programmable memory (believe it or not, it's also faster this way)

### Other notes
* When using Arduino IDE, replace the Digispark-shipped `micronucleus` with the one you built. 
* `micronucleus` repo has bootrom upgrades that can be used to update your `attiny85`
* Updated `attiny85` won't work with obsolete version shipped by Digispark.