fn main() {
    convert_strings_to_pig_latin();
}

fn convert_strings_to_pig_latin() {
    // Challenge:
    // Convert strings to pig latin.
    // Keep in mind the details about UTF-8 encoding!

    let s = String::from("Test string");
    let mut m: Vec<String> = vec![];

    for word in s.split_whitespace() {
        // Todo: Handle first words as vowels
        let mut latin_word = word[1..].to_owned();
        let first = &word[0..1];
        latin_word = latin_word + "-" + first + "ay";
        m.push(latin_word);
    }

    let output = m.join(" ");

    println!("{}", output);
}
