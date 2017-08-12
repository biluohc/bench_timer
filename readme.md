[![Build status](https://travis-ci.org/biluohc/bench_timer.svg?branch=master)](https://github.com/biluohc/bench_timer)
[![Latest version](https://img.shields.io/crates/v/bench_timer.svg)](https://crates.io/crates/bench_timer)
[![All downloads](https://img.shields.io/crates/d/bench_timer.svg)](https://crates.io/crates/bench_timer)
[![Downloads of latest version](https://img.shields.io/crates/dv/bench_timer.svg)](https://crates.io/crates/bench_timer)
[![Documentation](https://docs.rs/bench_timer/badge.svg)](https://docs.rs/bench_timer)

# bench_timer
## bench library for rust

### Notice: **Have to use release mod(with optimizations)**

### example:

#### on cargo.toml:

```toml
[dependencies]
stderr = "0.8.0"
bench_timer = "0.1.0"
```

or

```toml
[dependencies]
stderr = "0.8.0"
bench_timer = { git = "https://github.com/biluohc/bench_timer", branch = "master", version = "0.1.0"}
```

#### on code:

```rust
#[macro_use]
extern crate stderr;
#[macro_use]
extern crate bench_timer;

fn main() {
    let msg = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("fuck u".to_owned());

    timer_sort!(3,
                100000,
                to_owed(),
                new(),
                default(),
                format(),
                insert_e(&msg),
                push_str(&msg),
                format_s(&msg),
                insert_0(&msg),
                mul_args_add(&msg, ": cargo"),
                mul_args_like_push_str(&msg, ": cargo")
                );
}

#[inline(always)]
fn to_owed() {
    let s = "".to_owned();
    dbln!("{}", s);
}
#[inline(always)]
fn new() {
    let s = String::new();
    dbln!("{}", s);
}
#[inline(always)]
fn default() {
    let s = String::default();
    dbln!("{}", s);
}
#[inline(always)]
fn format() {
    let s = format!("{}", "");
    dbln!("{}", s);
}
#[inline(always)]
fn push_str(msg: &str) {
    let mut s = msg.to_owned();
    s.push_str(": cargo");
    dbln!("{}", s);
}

#[inline(always)]
fn format_s(msg: &str) {
    let s = format!("{}: cargo", msg);
    dbln!("{}", s);
}
#[inline(always)]
fn insert_0(msg: &str) {
    let mut s = ": cargo".to_owned();
    s.insert_str(0, msg);
    dbln!("{}", s);
}
#[inline(always)]
fn insert_e(msg: &str) {
    let mut s = ": cargo".to_owned();
    let len = s.len();
    s.insert_str(len, msg);
    dbln!("{}", s);
}
#[inline(always)]
fn mul_args_add(msg0: &str, msg1: &str) {
    let s = msg0.to_owned() + msg1;
    dbln!("{}", s);
}
#[inline(always)]
fn mul_args_like_push_str(msg0: &str, msg1: &str) {
    let mut s = msg0.to_owned();
    s.push_str(msg1);
    dbln!("{}", s);
}
```
