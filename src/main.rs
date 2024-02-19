// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn pig_latin(word: &str) -> Option<String> {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) => {
            if is_vowel(c) {
                Some(format!("{word}-hay"))
            } else {
                Some(format!("{}-{c}ay", chars.as_str()))
            }
        }
        None => None,
    }
}

fn main() {
    let word = String::from("Здравствуйте");
    // let word = String::from("first");
    // let word = String::from("apple");

    match pig_latin(&word) {
        Some(translated_word) => println!(
            "The word converted into pig-latin is \"{}\"",
            translated_word
        ),
        None => println!("The string is empty"),
    }
}
