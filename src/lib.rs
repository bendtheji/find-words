//! A rust library for finding words that can be constructed from a count-sensitive list of letters.
//!
//! ## Description
//! Given a list of English words (`words.txt`) and a list of letters (e.g. "yxmiasdaegwyxmiasdaegwyxmiasdaegw"), display all the words that can be constructed from any subset (or all) of those letters in any order.
//!
//! Note: This list of letters is count-sensitive, i.e. if your input list is 'bde' you can form the word 'bed' but you cannot form the word 'deed' as you only have one 'e'.
//!
//! ## Example
//! ```ignore
//! fn main() -> Result<(), std::io::Error> {
//!    let words = read_words_from_file("words.txt")?;
//!    let list = generate_random_string(Some(20));
//!    println!("List of letters: {}", list);
//!    println!("Words that can be constructed");
//!    for word  in get_constructable_words(words, &get_letters_count(&list)) {
//!        println!("{}", word)
//!    }
//!    Ok(())
//! }
//! ```
//!

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter;

use rand::prelude::*;
use rayon::prelude::*;

/// Struct that contains the word and the mapping of characters
/// that make up the word.
#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    pub value: String,
    pub letters: HashMap<char, u8>,
}

/// Returns a `HashMap` containing the character count mapping for a given word.
pub fn get_letters_count(word: &str) -> HashMap<char, u8> {
    word.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            let count = map.entry(c).or_insert(0);
            *count += 1;
            map
        })
}

/// Compare a `HashMap` containing the character count mapping for a word to the list of random letters.
fn can_be_constructed(word: &HashMap<char, u8>, list: &HashMap<char, u8>) -> bool {
    if word.is_empty() { return false; }
    word.iter().all(|(letter, letter_count)| match list.get(letter) {
        Some(list_letter_count) if list_letter_count >= letter_count => true,
        _ => false
    })
}

/// Read words from a file and puts them into a vector containing `Word` structs
pub fn read_words_from_file(file: &str) -> Result<Vec<Word>, Error> {
    let mut words = vec![];

    let file = File::open(file)?;
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let value = line?;
        let letters = get_letters_count(&value);
        words.push(Word { value, letters });
    }

    Ok(words)
}


/// Retrieve the constructable `Word` objects from the list
pub fn get_constructable_words(words: Vec<Word>, list: &HashMap<char, u8>) -> Vec<String> {
    words.into_par_iter()
        .filter_map(|Word { value, letters }|
            if can_be_constructed(&letters, &list) { Some(value) } else { None }
        )
        .collect()
}

/// Used to generate a random string given a length as an input. If `None` is passed in,
/// a random string of possible length from 1 to 200 is generated.
pub fn generate_random_string(length: Option<u8>) -> String {
    let mut rng = thread_rng();
    let length = length.unwrap_or_else(|| rng.gen_range(1..=200));
    iter::repeat_with(|| rng.gen_range('a'..='z')).take(length as usize).collect()
}

#[cfg(test)]
mod get_letters_count_tests {
    use std::collections::HashMap;

    use super::get_letters_count;

    #[test]
    fn empty_string() {
        let output = get_letters_count("");
        let expected = HashMap::new();
        assert_eq!(output, expected);
    }

    #[test]
    fn only_lowercase() {
        let output = get_letters_count("dodge");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn only_uppercase() {
        let output = get_letters_count("DODGE");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn mixed_case() {
        let output = get_letters_count("doDGe");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn alphanumeric() {
        let output = get_letters_count("dod123ge");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn contains_whitespaces() {
        let output = get_letters_count("\t   dod   ge\t\t\n  ");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    fn contains_punctuation() {
        let output = get_letters_count("do,dg!#$%^e");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }

    #[test]
    #[should_panic]
    fn wrong_count() {
        let output = get_letters_count("dodgy");
        let expected = HashMap::from([
            ('d', 2),
            ('o', 1),
            ('g', 1),
            ('e', 1)
        ]);
        assert_eq!(output, expected);
    }
}

#[cfg(test)]
mod can_be_constructed_tests {
    use crate::{can_be_constructed, get_letters_count};

    #[test]
    fn word_can_be_constructed() {
        let word = get_letters_count("dog");
        let list = get_letters_count("dodge");

        assert!(can_be_constructed(&word, &list));
    }

    #[test]
    #[should_panic]
    fn word_cannot_be_constructed() {
        let word = get_letters_count("dodgy");
        let list = get_letters_count("dodge");

        assert!(can_be_constructed(&word, &list));
    }

    #[test]
    #[should_panic]
    fn empty_list() {
        let word = get_letters_count("something");
        let list = get_letters_count("");

        assert!(can_be_constructed(&word, &list));
    }

    #[test]
    #[should_panic]
    fn empty_string_for_word() {
        let word = get_letters_count("");
        let list = get_letters_count("list");

        assert!(can_be_constructed(&word, &list));
    }
}

#[cfg(test)]
mod generate_random_string_tests {
    use crate::generate_random_string;

    #[test]
    fn length_of_4() {
        let output = generate_random_string(Some(4));
        let expected = 4;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn length_of_8() {
        let output = generate_random_string(Some(8));
        let expected = 8;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn length_of_20_string() {
        let output = generate_random_string(Some(20));
        let expected = 20;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn length_of_50_string() {
        let output = generate_random_string(Some(50));
        let expected = 50;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn length_of_100_string() {
        let output = generate_random_string(Some(100));
        let expected = 100;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn length_of_200_string() {
        let output = generate_random_string(Some(200));
        let expected = 200;
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }

    #[test]
    fn input_none() {
        let output = generate_random_string(None);
        let expected = output.len();
        assert_eq!(output.chars().filter(|c| c.is_ascii_alphabetic()).count(), expected);
    }
}

#[cfg(test)]
mod read_words_from_file_tests {
    use std::collections::HashMap;
    use std::env;

    use crate::{read_words_from_file, Word};

    fn get_file_path() -> String {
        let path = env::current_dir().unwrap();
        format!("{}/src/test_read_from_file.txt", path.display())
    }

    #[test]
    fn successfully_read() {
        let filename = get_file_path();
        let output = read_words_from_file(&filename).unwrap();
        let expected = vec![
            Word {
                value: String::from("cow"),
                letters: HashMap::from([
                    ('c', 1),
                    ('o', 1),
                    ('w', 1)
                ]),
            },
            Word {
                value: String::from("dog"),
                letters: HashMap::from([
                    ('d', 1),
                    ('o', 1),
                    ('g', 1)
                ]),
            },
            Word {
                value: String::from("milk"),
                letters: HashMap::from([
                    ('m', 1),
                    ('i', 1),
                    ('l', 1),
                    ('k', 1)
                ]),
            },
            Word {
                value: String::from("cat"),
                letters: HashMap::from([
                    ('c', 1),
                    ('a', 1),
                    ('t', 1)
                ]),
            },
            Word {
                value: String::from("goat"),
                letters: HashMap::from([
                    ('g', 1),
                    ('o', 1),
                    ('a', 1),
                    ('t', 1)
                ]),
            },
            Word {
                value: String::from("mammal"),
                letters: HashMap::from([
                    ('m', 3),
                    ('a', 2),
                    ('l', 1)
                ]),
            },
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn file_does_not_exist() {
        let filename = "invalid_file.txt";
        let output = read_words_from_file(filename);
        assert!(output.is_err());
    }
}