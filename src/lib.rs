/*!
## Notice: **Have to use release mod(with optimizations)**

## example: 

### on cargo.toml:

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

### on code: 

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
*/
use std::time::{SystemTime, SystemTimeError, Duration};

pub fn timer_avg<F>(mut f: F, times: u32) -> Result<Duration, SystemTimeError>
    where F: FnMut()
{
    let st = SystemTime::now();
    for _ in 0..times {
        f();
    }
    SystemTime::now().duration_since(st).map(|t| t / times)
}

#[macro_export]
macro_rules! timer_times {
     ($times_macro:expr, $times_fn:expr, $($fn_name: ident($($args: expr),*)),+) => {
         for _ in 0..$times_macro {
             $(println!("{}: {:?}",stringify!($fn_name), $crate::timer_avg(||$fn_name($($args),*), $times_fn).unwrap()); )+
             println!();
         }
     }
}

#[macro_export]
macro_rules! timer_sort {
    // tt 貌似也可以，只是也不能支持通过类型调用函数。
     ($times_macro:expr, $times_fn:expr, $($fn_name: tt($($args: expr),*)),+) => {
         use std::time::Duration;
         for _ in 0..$times_macro {
             let mut vs:Vec<(&str, Duration)> =Vec::default();
             $(vs.push((stringify!($fn_name),$crate::timer_avg(||$fn_name($($args),*), $times_fn).unwrap()));)+   
            vs.sort_by(|&(_,ref a),&(_,ref b)| b.cmp(a));
            let max_len= vs.as_slice().iter() .fold(0,| acc, &(s,_) | if s.len() > acc { s.len() } else { acc });            
            let blanks_fix= |msg: &str| {
                let mut tmp =String::default();
                for _ in 0..max_len- msg.len()
                 {tmp.push(' ');}
                tmp
            };
            vs.iter().map(|&(fn_name,t)|
            println!("{}{}: {:?}",fn_name,blanks_fix(fn_name),t)
             ).count();
             println!();
         }
     }
}
