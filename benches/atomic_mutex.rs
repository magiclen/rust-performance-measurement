#[macro_use]
extern crate bencher;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

use bencher::Bencher;

const MULTIPLE_COUNTER_NUMBER: usize = 4;

struct Counter {
    count: u64,
}

fn atomic(bencher: &mut Bencher) {
    let number = AtomicU64::new(0);

    bencher.iter(|| {
        number.fetch_add(1, Ordering::SeqCst);
    });
}

fn mutex(bencher: &mut Bencher) {
    let number = Mutex::new(Counter {
        count: 0
    });

    bencher.iter(|| {
        number.lock().unwrap().count += 1;
    });
}

fn atomic_multiple(bencher: &mut Bencher) {
    let mut numbers = Vec::with_capacity(MULTIPLE_COUNTER_NUMBER);

    for _ in 0..MULTIPLE_COUNTER_NUMBER {
        numbers.push(AtomicU64::new(0));
    }

    bencher.iter(|| {
        for number in numbers.iter_mut() {
            number.fetch_add(1, Ordering::SeqCst);
        }
    });
}

fn mutex_multiple(bencher: &mut Bencher) {
    let mut numbers = Vec::with_capacity(MULTIPLE_COUNTER_NUMBER);

    for _ in 0..MULTIPLE_COUNTER_NUMBER {
        numbers.push(Counter {
            count: 0
        });
    }

    let numbers = Mutex::new(numbers);

    bencher.iter(|| {
        let mut numbers = numbers.lock().unwrap();

        for number in numbers.iter_mut() {
            number.count += 1;
        }
    });
}

benchmark_group!(atomic_mutex, atomic, mutex);
benchmark_group!(atomic_mutex_multiple, atomic_multiple, mutex_multiple);

benchmark_main!(atomic_mutex, atomic_mutex_multiple);
