#[macro_use]
extern crate bencher;

extern crate random_integer;

use bencher::Bencher;

use random_integer::{rand, random_u8_with_rng, random_usize_with_rng};

static NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

static LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static LETTERS: [char; 52] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static SYMBOLS: [char; 32] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

static NONSENSE: [&str; 26] = [
    "ghostsed",
    "ghostses",
    "ghostsghosts",
    "ghostsing",
    "ghostsstsohg",
    "ghosttsohg",
    "ghst",
    "gianT",
    "giannI",
    "gianni",
    "gianni!",
    "gianni.",
    "gianni0",
    "gianni1",
    "gianni2",
    "gianni3",
    "gianni4",
    "gianni5",
    "gianni6",
    "gianni7",
    "gianni8",
    "gianni9",
    "gianni?",
    "giannied",
    "giannigianni",
    "gianniinnaig",
];

fn contains_is_number(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        NUMBERS.contains(&needle)
    });
}

fn binary_search_is_number(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        NUMBERS.binary_search(&needle).is_ok()
    });
}

fn contains_is_lowercase(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        LOWERCASE_LETTERS.contains(&needle)
    });
}

fn binary_search_is_lowercase(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        LOWERCASE_LETTERS.binary_search(&needle).is_ok()
    });
}

fn contains_is_symbol(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        SYMBOLS.contains(&needle)
    });
}

fn binary_search_is_symbol(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        SYMBOLS.binary_search(&needle).is_ok()
    });
}

fn contains_is_letter(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        LETTERS.contains(&needle)
    });
}

fn binary_search_is_letter(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let needle = random_u8_with_rng(0, 127, &mut rng) as char;

        LETTERS.binary_search(&needle).is_ok()
    });
}

fn contains_is_non_sense(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let index = random_usize_with_rng(0, NONSENSE.len() * 2, &mut rng);

        let needle = if index < NONSENSE.len() {
            NONSENSE[index]
        } else {
            "1bigrep"
        };

        NONSENSE.contains(&needle)
    });
}

fn binary_search_is_non_sense(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();

    bencher.iter(|| {
        let index = random_usize_with_rng(0, NONSENSE.len() * 2, &mut rng);

        let needle = if index < NONSENSE.len() {
            NONSENSE[index]
        } else {
            "1bigrep"
        };

        NONSENSE.binary_search(&needle).is_ok()
    });
}

benchmark_group!(
    contains,
    contains_is_number,
    contains_is_lowercase,
    contains_is_symbol,
    contains_is_letter,
    contains_is_non_sense,
);
benchmark_group!(
    binary_search,
    binary_search_is_number,
    binary_search_is_lowercase,
    binary_search_is_symbol,
    binary_search_is_letter,
    binary_search_is_non_sense,
);

benchmark_main!(contains, binary_search);
