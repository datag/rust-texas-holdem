# rust-texas-holdem

This is a tinkering/learning project aimed at reimplementing some of the functionality of my
[HoldingNuts libpoker](https://github.com/holdingnuts/holdingnuts/tree/master/src/libpoker)
(which was written in C++/Qt) in Rust.

It currently consists of:
* Handling of playing cards
* Handling of a card deck
* Simulating Texas Holdem hands and comparing with their probabilities

## Run

```shell
# Debug run
cargo run

# Release run
cargo run --release
```
