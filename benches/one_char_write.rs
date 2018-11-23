#[macro_use]
extern crate criterion;

use std::fmt::{self, Display, Formatter, Write};

use criterion::{Criterion, Benchmark};

struct MyWriteStr;

impl Display for MyWriteStr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("\n")
    }
}

struct MyWrite;

impl Display for MyWrite {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_char('\n')
    }
}

struct MyWriteStrExt;

impl Display for MyWriteStrExt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("囉")
    }
}

struct MyWriteExt;

impl Display for MyWriteExt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_char('囉')
    }
}

fn write_str(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_write",
        Benchmark::new("write_str", move |b| {
            b.iter(|| {
                s.write_fmt(format_args!("{}", MyWriteStr)).unwrap();
            });
        }),
    );
}

fn write_char(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_write",
        Benchmark::new("write_char", move |b| {
            b.iter(|| {
                s.write_fmt(format_args!("{}", MyWrite)).unwrap();
            });
        }),
    );
}

fn write_str_ext(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_write",
        Benchmark::new("write_str_ext", move |b| {
            b.iter(|| {
                s.write_fmt(format_args!("{}", MyWriteStrExt)).unwrap();
            });
        }),
    );
}

fn write_char_ext(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_write",
        Benchmark::new("write_char_ext", move |b| {
            b.iter(|| {
                s.write_fmt(format_args!("{}", MyWriteExt)).unwrap();
            });
        }),
    );
}


criterion_group!(one_char_write, write_str, write_char, write_str_ext,write_char_ext );

criterion_main!(one_char_write);