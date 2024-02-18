// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn main() {
    let mut word = String::from("Здравствуйте");

    let mut chars = word.chars();
    match chars.next() {
        Some(c) => match is_vowel(c) {
            true => word.push_str("-hay"),
            _ => {
                let mut result = String::new();
                chars.for_each(|character| result.push(character));
                result.push_str("-");
                result.push(c);
                result.push_str("ay");
                word = result;
            }
        },
        None => println!("The string is empty"),
    }

    println!("The word converted into pig-latin is \"{}\"", word)
}
