#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

use std::sync::atomic::{Ordering, AtomicU64};
use std::sync::Mutex;

const MULTIPLE_COUNTER_NUMBER: usize = 4;

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

fn atomic_multiple(c: &mut Criterion) {
    c.bench(
        "atomic_mutex_multiple",
        Benchmark::new("atomic_multiple", move |b| {
            let mut numbers = Vec::with_capacity(MULTIPLE_COUNTER_NUMBER);

            for _ in 0..MULTIPLE_COUNTER_NUMBER {
                numbers.push(AtomicU64::new(0));
            }

            b.iter(|| {
                for number in numbers.iter_mut() {
                    number.fetch_add(1, Ordering::SeqCst);
                }
            });
        }),
    );
}

fn mutex_multiple(c: &mut Criterion) {
    c.bench(
        "atomic_mutex_multiple",
        Benchmark::new("mutex_multiple", move |b| {
            let mut numbers = Vec::with_capacity(MULTIPLE_COUNTER_NUMBER);

            for _ in 0..MULTIPLE_COUNTER_NUMBER {
                numbers.push(Counter {
                    count: 0
                });
            }

            let numbers = Mutex::new(numbers);

            b.iter(|| {
                let mut numbers = numbers.lock().unwrap();

                for number in numbers.iter_mut() {
                    number.count += 1;
                }
            });
        }),
    );
}

criterion_group!(atomic_mutex, atomic, mutex);
criterion_group!(atomic_mutex_multiple, atomic_multiple, mutex_multiple);

criterion_main!(atomic_mutex, atomic_mutex_multiple);