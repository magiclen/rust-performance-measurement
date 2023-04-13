#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn push_str(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        #[allow(clippy::single_char_add_str)]
        s.push_str("\n");
    });
}

fn push(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.push('\n');
    });
}

fn push_str_ext(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        #[allow(clippy::single_char_add_str)]
        s.push_str("哈");
    });
}

fn push_ext(bencher: &mut Bencher) {
    let mut s = String::new();

    bencher.iter(|| {
        s.push('哈');
    });
}

benchmark_group!(one_char_push, push_str, push, push_str_ext, push_ext);

benchmark_main!(one_char_push);
