use std::env;
use std::time::Duration;

use criterion::{BatchSize, Bencher, black_box, Criterion, criterion_group, criterion_main};

use find_words::{generate_random_string, get_constructable_words, get_letters_count, read_words_from_file};

fn bench_find_words_in_letter_string(b: &mut Bencher, list_length: u8, filename: &str) {
    let words = black_box(read_words_from_file(filename).unwrap());
    let list = black_box(get_letters_count(&generate_random_string(Some(list_length))));
    b.iter_batched(|| words.to_vec(),
                   |words| get_constructable_words(words, &list),
                   BatchSize::SmallInput,
    )
}

fn find_words_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find words");
    group.measurement_time(Duration::new(6, 0));
    let no_of_words = [100u16, 1000u16, 10000u16];
    let various_lengths = [4, 8, 12, 50, 100, 200];
    for curr_no in no_of_words {
        let path = get_file_path(curr_no);
        various_lengths.iter().for_each(|&length| {
            group.bench_function(
                format!("bench find words in {} letter string from {} words", length, curr_no),
                |b| bench_find_words_in_letter_string(b, length, &path),
            );
        });
    }
}

fn get_file_path(no_of_words: u16) -> String {
    let path = env::current_dir().unwrap();
    let filename = format!("benchmark_{}_words.txt", no_of_words);
    format!("{}/benches/{}", path.display(), filename)
}

criterion_group!(benches, find_words_benchmark);
criterion_main!(benches);