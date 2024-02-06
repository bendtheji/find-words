use find_words::{generate_random_string, get_constructable_words, get_letters_count, read_words_from_file};

fn main() -> Result<(), std::io::Error> {
    let words = read_words_from_file("words.txt")?;
    let list = generate_random_string(Some(20));
    println!("List of letters: {}", list);
    println!("Words that can be constructed");
    for word  in get_constructable_words(words, &get_letters_count(&list)) {
        println!("{}", word)
    }
    Ok(())
}