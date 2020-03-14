#[macro_use]
extern crate bencher;

use bencher::Bencher;

static STATIC_INT_ARRAY: [u64; 200] = [
    0x4AE0, 0xA570, 0x5268, 0xD260, 0xD950, 0x6AA8, 0x56A0, 0x9AD0, 0x4AE8, 0x4AE0,
    0xA4D8, 0xA4D0, 0xD250, 0xD528, 0xB540, 0xD6A0, 0x96D0, 0x95B0, 0x49B8, 0x4970,
    0xA4B0, 0xB258, 0x6A50, 0x6D40, 0xADA8, 0x2B60, 0x9570, 0x4978, 0x4970, 0x64B0,
    0xD4A0, 0xEA50, 0x6D48, 0x5AD0, 0x2B60, 0x9370, 0x92E0, 0xC968, 0xC950, 0xD4A0,
    0xDA50, 0xB550, 0x56A0, 0xAAD8, 0x25D0, 0x92D0, 0xC958, 0xA950, 0xB4A8, 0x6CA0,
    0xB550, 0x55A8, 0x4DA0, 0xA5B0, 0x52B8, 0x52B0, 0xA950, 0xE950, 0x6AA0, 0xAD50,
    0xAB50, 0x4B60, 0xA570, 0xA570, 0x5260, 0xE930, 0xD950, 0x5AA8, 0x56A0, 0x96D0,
    0x4AE8, 0x4AD0, 0xA4D0, 0xD268, 0xD250, 0xD528, 0xB540, 0xB6A0, 0x96D0, 0x95B0,
    0x49B0, 0xA4B8, 0xA4B0, 0xB258, 0x6A50, 0x6D40, 0xADA0, 0xAB60, 0x9570, 0x4978,
    0x4970, 0x64B0, 0x6A50, 0xEA50, 0x6B28, 0x5AC0, 0xAB60, 0x9368, 0x92E0, 0xC960,
    0xD4A8, 0xD4A0, 0xDA50, 0x5AA8, 0x56A0, 0xAAD8, 0x25D0, 0x92D0, 0xC958, 0xA950,
    0xB4A0, 0xB550, 0xAD50, 0x55A8, 0x4BA0, 0xA5B0, 0x52B8, 0x52B0, 0xA930, 0x74A8,
    0x6AA0, 0xAD50, 0x4DA8, 0x4B60, 0xA570, 0xA4E0, 0xD260, 0xE930, 0xD530, 0x5AA0,
    0x6B50, 0x96D0, 0x4AE8, 0x4AD0, 0xA4D0, 0xD258, 0xD250, 0xD520, 0xDAA0, 0xB5A0,
    0x56D0, 0x4AD8, 0x49B0, 0xA4B8, 0xA4B0, 0xAA50, 0xB528, 0x6D20, 0xADA0, 0x55B0,
    0x9370, 0x4978, 0x4970, 0x64B0, 0x6A50, 0xEA50, 0x6B20, 0xAB60, 0xAAE0, 0x92E0,
    0xC970, 0xC960, 0xD4A8, 0xD4A0, 0xDA50, 0x5AA8, 0x56A0, 0xA6D0, 0x52E8, 0x52D0,
    0xA958, 0xA950, 0xB4A0, 0xB550, 0xAD50, 0x55A0, 0xA5D0, 0xA5B0, 0x52B0, 0xA938,
    0x6930, 0x7298, 0x6AA0, 0xAD50, 0x4DA8, 0x4B60, 0xA570, 0x5270, 0xD260, 0xE930,
    0xD520, 0xDAA0, 0x6B50, 0x56D0, 0x4AE0, 0xA4E8, 0xA4D0, 0xD150, 0xD928, 0xD520,
];

fn range(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut sum = 0;

        for i in 0..STATIC_INT_ARRAY.len() {
            sum += i as u64 + STATIC_INT_ARRAY[i];
        }

        sum
    });
}

fn enumerate(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut sum = 0;

        for (i, &n) in STATIC_INT_ARRAY.iter().enumerate() {
            sum += i as u64 + n;
        }

        sum
    });
}

benchmark_group!(iter_skip_take, range, enumerate);

benchmark_main!(iter_skip_take);