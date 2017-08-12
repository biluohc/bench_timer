#[macro_use]
extern crate stderr;
#[macro_use]
extern crate bench_timer;

#[test]
fn main() {
    let msg = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("fuck u".to_owned());
    timer_times!(3,
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
                 mul_args_like_push_str(&msg, ": cargo"));
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
                mul_args_like_push_str(&msg, ": cargo"));
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