#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

// if the array size is big enough, `no_box`, `with_box_2`, `alloc_no_box`, `alloc_with_box_2` functions will cause stack overflow.
const ARRAY_SIZE: usize = 1_000_000;
static mut ARRAY_SIZE_IN_STACK: usize = ARRAY_SIZE;

fn no_box(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "vec_box_array",
        Benchmark::new("no_box", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|e| *e = 255);
                a.iter_mut().for_each(|e| *e = 0);
            });
        }),
    );
}

fn with_box(c: &mut Criterion) {
    let mut a = {
        let v = vec![0u8; ARRAY_SIZE];

        let a = v.into_boxed_slice();

        unsafe {
            Box::from_raw(Box::into_raw(a) as *mut [u8; ARRAY_SIZE])
        }
    };

    c.bench(
        "vec_box_array",
        Benchmark::new("with_box", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|e| *e = 255);
                a.iter_mut().for_each(|e| *e = 0);
            });
        }),
    );
}

fn with_box_2(c: &mut Criterion) {
    let mut a = Box::new([0u8; ARRAY_SIZE]);

    c.bench(
        "vec_box_array",
        Benchmark::new("with_box_2", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|e| *e = 255);
                a.iter_mut().for_each(|e| *e = 0);
            });
        }),
    );
}

fn with_vec(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "vec_box_array",
        Benchmark::new("with_vec", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|e| *e = 255);
                a.iter_mut().for_each(|e| *e = 0);
            });
        }),
    );
}

fn with_vec_to_box(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE].into_boxed_slice();

    c.bench(
        "vec_box_array",
        Benchmark::new("with_vec_to_box", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|e| *e = 255);
                a.iter_mut().for_each(|e| *e = 0);
            });
        }),
    );
}

fn alloc_no_box(c: &mut Criterion) {
    c.bench(
        "alloc_vec_box_array",
        Benchmark::new("alloc_no_box", move |b| {
            b.iter(|| {
                let a = [0u8; ARRAY_SIZE];

                unsafe {
                    assert_eq!(ARRAY_SIZE_IN_STACK, a.len());
                }
            });
        }),
    );
}

fn alloc_with_box(c: &mut Criterion) {
    c.bench(
        "alloc_vec_box_array",
        Benchmark::new("alloc_with_box", move |b| {
            b.iter(|| {
                let a = {
                    let v = vec![0u8; ARRAY_SIZE];

                    let a = v.into_boxed_slice();

                    unsafe {
                        Box::from_raw(Box::into_raw(a) as *mut [u8; ARRAY_SIZE])
                    }
                };

                unsafe {
                    assert_eq!(ARRAY_SIZE_IN_STACK, a.len());
                }
            });
        }),
    );
}

fn alloc_with_box_2(c: &mut Criterion) {
    c.bench(
        "alloc_vec_box_array",
        Benchmark::new("alloc_with_box_2", move |b| {
            b.iter(|| {
                let a = Box::new([0u8; ARRAY_SIZE]);

                unsafe {
                    assert_eq!(ARRAY_SIZE_IN_STACK, a.len());
                }
            });
        }),
    );
}

fn alloc_with_vec(c: &mut Criterion) {
    c.bench(
        "alloc_vec_box_array",
        Benchmark::new("alloc_with_vec", move |b| {
            b.iter(|| {
                let a = vec![0u8; ARRAY_SIZE];

                unsafe {
                    assert_eq!(ARRAY_SIZE_IN_STACK, a.len());
                }
            });
        }),
    );
}

fn alloc_with_vec_to_box(c: &mut Criterion) {
    c.bench(
        "alloc_vec_box_array",
        Benchmark::new("alloc_with_vec_to_box", move |b| {
            b.iter(|| {
                let a = vec![0u8; ARRAY_SIZE].into_boxed_slice();

                unsafe {
                    assert_eq!(ARRAY_SIZE_IN_STACK, a.len());
                }
            });
        }),
    );
}

criterion_group!(modify, no_box, with_box, with_box_2, with_vec, with_vec_to_box);

criterion_group!(allocation, alloc_no_box, alloc_with_box, alloc_with_box_2, alloc_with_vec, alloc_with_vec_to_box);

criterion_main!(allocation, modify);