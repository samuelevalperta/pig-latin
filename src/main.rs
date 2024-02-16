// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
fn main() {
    let mut word = String::from("apple");

    match word.chars().next() {
        Some(c) => match is_vowel(c) {
            true => word.push_str("-hay"),
            _ => {
                todo!()
                // let mut result = String::new();
                // for d in word.chars().next() {
                // result.add(d)
                // }
            }
        },
        None => println!("The string is empty"),
    }

    println!("The word converted into pig-latin is \"{}\"", word)
}
