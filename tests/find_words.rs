use find_words::{get_constructable_words, get_letters_count, read_words_from_file};

#[test]
fn find_words_in_4_letter_list() {
    let list = "cmbl";
    let words = read_words_from_file("words.txt");
    let expected: Vec<String> = vec![];
    let output = match words {
        Ok(words) => get_constructable_words(words, &get_letters_count(list)),
        _err => panic!("problem reading file")
    };
    assert_eq!(output, expected);
}

#[test]
fn find_words_in_8_letter_list() {
    let list = "wartsmrf";
    let words = read_words_from_file("words.txt");
    let expected: Vec<String> = vec!["arm", "art"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let output = match words {
        Ok(words) => get_constructable_words(words, &get_letters_count(list)),
        _err => panic!("problem reading file")
    };
    assert_eq!(output, expected);
}

#[test]
fn find_words_in_20_letter_list() {
    let list = "fsucwcaumvxvkfvpbkjw";
    let words = read_words_from_file("words.txt");
    let expected: Vec<String> = vec!["back", "camp", "cap", "cub", "cup"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let output = match words {
        Ok(words) => get_constructable_words(words, &get_letters_count(list)),
        _err => panic!("problem reading file")
    };
    assert_eq!(output, expected);
}