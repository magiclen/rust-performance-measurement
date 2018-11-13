#[macro_use]
extern crate criterion;

use std::fmt::Write;

use criterion::{Criterion, Benchmark};

static mut ANY_NUMBER: f64 = 45.48;

fn push_str(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "push_str_write_fmt",
        Benchmark::new("push_str", move |b| {
            b.iter(|| {
                s.push_str(&format!("Hello! {}.", unsafe { ANY_NUMBER }));
            });
        }),
    );
}

fn write_fmt(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "push_str_write_fmt",
        Benchmark::new("write_fmt", move |b| {
            b.iter(|| {
                s.write_fmt(format_args!("Hello! {}.", unsafe { ANY_NUMBER })).unwrap();
            });
        }),
    );
}


criterion_group!(push_str_write_fmt, push_str, write_fmt);

criterion_main!(push_str_write_fmt);