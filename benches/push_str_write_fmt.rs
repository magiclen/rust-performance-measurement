#[macro_use]
extern crate bencher;

use std::fmt::Write;

use bencher::Bencher;

static mut ANY_NUMBER: f64 = 45.48;

fn push_str(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.push_str(&format!("Hello! {}.", unsafe { ANY_NUMBER }));
    });
}

fn write_fmt(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.write_fmt(format_args!("Hello! {}.", unsafe { ANY_NUMBER })).unwrap();
    });
}

benchmark_group!(push_str_write_fmt, push_str, write_fmt);

benchmark_main!(push_str_write_fmt);
