// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn pig_latin(word: &str) -> Option<String> {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) => {
            if is_vowel(c) {
                let mut result = word.to_string();
                result.push_str("-hay");
                Some(result)
            } else {
                let mut result = String::new();
                result.extend(chars);
                Some(format!("{result}-{c}ay"))
            }
        }
        None => None,
    }
}

fn main() {
    let word = String::from("Здравствуйте");

    match pig_latin(&word) {
        Some(translated_word) => println!(
            "The word converted into pig-latin is \"{}\"",
            translated_word
        ),
        None => println!("The string is empty"),
    }
}
