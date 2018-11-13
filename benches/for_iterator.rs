#[macro_use]
extern crate criterion;

use criterion::{Criterion, Benchmark};

const ARRAY_SIZE: usize = 1_000_000;

fn for_loop_stack(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_loop_stack", move |b| {
            b.iter(|| {
                for i in 0..ARRAY_SIZE {
                    a[i] = 255;
                }
                for i in 0..ARRAY_SIZE {
                    a[i] = 0;
                }
            });
        }),
    );
}

fn for_loop_stack_slice(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_loop_stack_slice", move |b| {
            let a = a.as_mut();
            b.iter(|| {
                for i in 0..ARRAY_SIZE {
                    a[i] = 255;
                }
                for i in 0..ARRAY_SIZE {
                    a[i] = 0;
                }
            });
        }),
    );
}

fn for_loop_heap(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_loop_heap", move |b| {
            b.iter(|| {
                for i in 0..ARRAY_SIZE {
                    a[i] = 255;
                }
                for i in 0..ARRAY_SIZE {
                    a[i] = 0;
                }
            });
        }),
    );
}

fn for_loop_heap_slice(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_loop_heap_slice", move |b| {
            let a = a.as_mut_slice();
            b.iter(|| {
                for i in 0..ARRAY_SIZE {
                    a[i] = 255;
                }
                for i in 0..ARRAY_SIZE {
                    a[i] = 0;
                }
            });
        }),
    );
}

fn iterator_stack(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("iterator_stack", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|a| *a = 255);
                a.iter_mut().for_each(|a| *a = 0);
            });
        }),
    );
}

fn iterator_stack_slice(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("iterator_stack_slice", move |b| {
            let a = a.as_mut();
            b.iter(|| {
                a.iter_mut().for_each(|a| *a = 255);
                a.iter_mut().for_each(|a| *a = 0);
            });
        }),
    );
}

fn iterator_heap(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("iterator_heap", move |b| {
            b.iter(|| {
                a.iter_mut().for_each(|a| *a = 255);
                a.iter_mut().for_each(|a| *a = 0);
            });
        }),
    );
}

fn iterator_heap_slice(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("iterator_heap_slice", move |b| {
            let a = a.as_mut_slice();
            b.iter(|| {
                a.iter_mut().for_each(|a| *a = 255);
                a.iter_mut().for_each(|a| *a = 0);
            });
        }),
    );
}

fn for_iterator_loop_stack(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_iterator_loop_stack", move |b| {
            b.iter(|| {
                for e in a.iter_mut() {
                    *e = 255;
                }
                for e in a.iter_mut() {
                    *e = 0;
                }
            });
        }),
    );
}

fn for_iterator_loop_stack_slice(c: &mut Criterion) {
    let mut a = [0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_iterator_loop_stack_slice", move |b| {
            let a = a.as_mut();
            b.iter(|| {
                for e in a.iter_mut() {
                    *e = 255;
                }
                for e in a.iter_mut() {
                    *e = 0;
                }
            });
        }),
    );
}

fn for_iterator_loop_heap(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_iterator_loop_heap", move |b| {
            b.iter(|| {
                for e in a.iter_mut() {
                    *e = 255;
                }
                for e in a.iter_mut() {
                    *e = 0;
                }
            });
        }),
    );
}

fn for_iterator_loop_heap_slice(c: &mut Criterion) {
    let mut a = vec![0u8; ARRAY_SIZE];

    c.bench(
        "for_loop",
        Benchmark::new("for_iterator_loop_heap_slice", move |b| {
            let a = a.as_mut_slice();
            b.iter(|| {
                for e in a.iter_mut() {
                    *e = 255;
                }
                for e in a.iter_mut() {
                    *e = 0;
                }
            });
        }),
    );
}

criterion_group!(stack, for_loop_stack, for_loop_stack_slice, iterator_stack, iterator_stack_slice, for_iterator_loop_stack, for_iterator_loop_stack_slice);

criterion_group!(heap, for_loop_heap, for_loop_heap_slice, iterator_heap, iterator_heap_slice, for_iterator_loop_heap, for_iterator_loop_heap_slice);

criterion_main!(stack, heap);