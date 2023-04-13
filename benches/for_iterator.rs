#[macro_use]
extern crate bencher;

use bencher::Bencher;

const ARRAY_SIZE: usize = 1_000_000;

fn for_loop_stack(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    bencher.iter(|| {
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 255;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 0;
        }
    });
}

fn for_loop_stack_slice(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 255;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 0;
        }
    });
}

fn for_loop_heap(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    bencher.iter(|| {
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 255;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 0;
        }
    });
}

fn for_loop_heap_slice(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 255;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..ARRAY_SIZE {
            a[i] = 0;
        }
    });
}

fn iterator_stack(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    bencher.iter(|| {
        a.iter_mut().for_each(|a| *a = 255);
        a.iter_mut().for_each(|a| *a = 0);
    });
}

fn iterator_stack_slice(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        a.iter_mut().for_each(|a| *a = 255);
        a.iter_mut().for_each(|a| *a = 0);
    });
}

fn iterator_heap(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    bencher.iter(|| {
        a.iter_mut().for_each(|a| *a = 255);
        a.iter_mut().for_each(|a| *a = 0);
    });
}

fn iterator_heap_slice(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        a.iter_mut().for_each(|a| *a = 255);
        a.iter_mut().for_each(|a| *a = 0);
    });
}

fn for_iterator_loop_stack(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    bencher.iter(|| {
        for e in a.iter_mut() {
            *e = 255;
        }
        for e in a.iter_mut() {
            *e = 0;
        }
    });
}

fn for_iterator_loop_stack_slice(bencher: &mut Bencher) {
    let mut a = [0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        for e in a.iter_mut() {
            *e = 255;
        }
        for e in a.iter_mut() {
            *e = 0;
        }
    });
}

fn for_iterator_loop_heap(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    bencher.iter(|| {
        for e in a.iter_mut() {
            *e = 255;
        }
        for e in a.iter_mut() {
            *e = 0;
        }
    });
}

fn for_iterator_loop_heap_slice(bencher: &mut Bencher) {
    let mut a = vec![0u8; ARRAY_SIZE];

    let a: &mut [u8] = a.as_mut();

    bencher.iter(|| {
        for e in a.iter_mut() {
            *e = 255;
        }
        for e in a.iter_mut() {
            *e = 0;
        }
    });
}

benchmark_group!(
    stack,
    for_loop_stack,
    for_loop_stack_slice,
    iterator_stack,
    iterator_stack_slice,
    for_iterator_loop_stack,
    for_iterator_loop_stack_slice
);
benchmark_group!(
    heap,
    for_loop_heap,
    for_loop_heap_slice,
    iterator_heap,
    iterator_heap_slice,
    for_iterator_loop_heap,
    for_iterator_loop_heap_slice
);

benchmark_main!(stack, heap);
