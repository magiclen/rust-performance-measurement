#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

use std::sync::atomic::{Ordering, AtomicU64};
use std::sync::Mutex;

struct Counter {
    count: u64
}

fn atomic(c: &mut Criterion) {
    c.bench(
        "atomic_mutex",
        Benchmark::new("atomic", move |b| {
            let number = AtomicU64::new(0);

            b.iter(|| {
                number.fetch_add(1, Ordering::SeqCst)
            });
        }),
    );
}

fn mutex(c: &mut Criterion) {
    c.bench(
        "atomic_mutex",
        Benchmark::new("mutex", move |b| {
            let number = Mutex::new(Counter {
                count: 0
            });

            b.iter(|| {
                number.lock().unwrap().count += 1;
            });
        }),
    );
}

criterion_group!(atomic_mutex, atomic, mutex);

criterion_main!(atomic_mutex);