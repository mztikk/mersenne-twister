# mersenne-twister
Mersenne Twister([MT19937](http://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html)) implementation in Rust. Found on crates as [mersenne-twister-m](https://crates.io/crates/mersenne-twister-m)

## Examples
Create MT19937 instance with either default seed
```rust
let mut mt = MT19937::default();
let mut mt = MT19937::new();
```

or a custom seed
```rust
let mut mt = MT19937::new_with_seed(4537);
```

generate or peek random value with
```rust
mt.genrand()
mt.peek()
```

## Build
Build with
```rust
cargo build
```

## Test
Run tests with
```rust
cargo test
```
