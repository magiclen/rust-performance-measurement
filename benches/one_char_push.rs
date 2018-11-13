#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

fn push_str(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_push",
        Benchmark::new("push_str", move |b| {
            b.iter(|| {
                s.push_str("\n");
            });
        }),
    );
}

fn push(c: &mut Criterion) {
    let mut s = String::new();

    c.bench(
        "one_char_push",
        Benchmark::new("push", move |b| {
            b.iter(|| {
                s.push('\n');
            });
        }),
    );
}


criterion_group!(one_char_push, push_str, push);

criterion_main!(one_char_push);