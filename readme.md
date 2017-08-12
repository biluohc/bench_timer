# timer
## bench library for rust

### Notice:

**Have to use release mod(with optimizations)**

### example:

#### on cargo.toml:

```toml
[dependencies]
stderr = "0.8.0"
timer = { git = "https://github.com/biluohc/timer", branch = "master", version = "0.1.0"}
```

#### on code:

```rust
#[macro_use]
extern crate stderr;
#[macro_use]
extern crate timer;

fn main() {
    let msg = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("fuck u".to_owned());

    timer_sort!(3,
                100000,
                format_s(&msg),
                insert_0(&msg),
                insert_e(&msg),
                push_str(&msg));
}

#[inline(always)]
fn push_str(msg: &str) {
    let mut s = msg.to_owned();
    s.push_str(": timer");
    dbln!("{}", s);
}

#[inline(always)]
fn format_s(msg: &str) {
    let s = format!("{}: timer", msg);
    dbln!("{}", s);
}
#[inline(always)]
fn insert_0(msg: &str) {
    let mut s = ": timer".to_owned();
    s.insert_str(0, msg);
    dbln!("{}", s);
}
#[inline(always)]
fn insert_e(msg: &str) {
    let mut s = ": timer".to_owned();
    let len = s.len();
    s.insert_str(len, msg);
    dbln!("{}", s);
}
```
