#[macro_use]
extern crate bencher;

use std::fmt::{self, Display, Formatter, Write};

use bencher::Bencher;

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

fn write_str(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.write_fmt(format_args!("{}", MyWriteStr)).unwrap();
    });
}

fn write_char(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.write_fmt(format_args!("{}", MyWrite)).unwrap();
    });
}

fn write_str_ext(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.write_fmt(format_args!("{}", MyWriteStrExt)).unwrap();
    });
}

fn write_char_ext(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.write_fmt(format_args!("{}", MyWriteExt)).unwrap();
    });
}


benchmark_group!(one_char_write, write_str, write_char, write_str_ext,write_char_ext );

benchmark_main!(one_char_write);