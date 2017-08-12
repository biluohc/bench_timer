/*!

example: 

on cargo.toml:

```toml
[dependencies]
stderr = "0.8.0"
timer = { git = "https://github.com/biluohc/timer", branch = "master", version = "0.1.0"}
```

on code: 

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
     ($times_macro:expr, $times_fn:expr, $($fn_name: ident($args: expr)),+) => {
         for _ in 0..$times_macro {
             $(println!("{}: {:?}",stringify!($fn_name), $crate::timer_avg(||$fn_name($args), $times_fn).unwrap()); )+
             println!();
         }
     }
}

#[macro_export]
macro_rules! timer_sort {
    // tt 貌似也可以，只是也不能支持通过类型调用函数。
     ($times_macro:expr, $times_fn:expr, $($fn_name: tt($args: expr)),+) => {
         use std::time::Duration;
         for _ in 0..$times_macro {
             let mut vs:Vec<(&str, Duration)> =Vec::default();
             $(vs.push((stringify!($fn_name),$crate::timer_avg(||$fn_name($args), $times_fn).unwrap()));)+   
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
