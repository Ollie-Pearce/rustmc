# rose_bloom

`rose_bloom` is a crate for passing out references that won't move when you push to it. It is also thread-safe, so you can use it as a concurrent queue if you don't care about freeing memory as you go along. It is lock-free.

Many operations are O(lg n). This will still be extremely fast.

## Example

```rust
use rose_bloom::Rose;

let rose = Rose::new();
let out1 = rose.push(1);
rose.push(2);
rose.push(3);
println!("{out1}"); // 1
```

## Installation

Add this to your Cargo.toml:
```toml
[dependencies]
rose_bloom = "0.1"
```

## `#![no_std]`

This crate is `#![no_std]` but does require `alloc`.

## Licenses

-   [MIT](https://choosealicense.com/licenses/mit/)
-   [Unlicense](https://choosealicense.com/licenses/unlicense/)