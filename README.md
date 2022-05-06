# mersenne-twister
Mersenne Twister(MT19937) implementation in Rust

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
