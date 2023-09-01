fn main() {
    convert_strings_to_pig_latin();
}

fn convert_strings_to_pig_latin() {
    // Challenge:
    // Convert strings to pig latin.
    // Keep in mind the details about UTF-8 encoding!

    let s = String::from("Est string");
    let mut m: Vec<String> = vec![];

    let vowels = vec!["a", "e", "i", "o", "u"];

    for word in s.split_whitespace() {
        let first = &word[0..1];
        let latin_word: String;
        if vowels.iter().any(|&l| l == first.to_lowercase()) {
            latin_word = word.to_owned() + "-hay";
        } else {
            latin_word = word[1..].to_owned() + "-" + first + "ay";
        }
        
        m.push(latin_word);
    }

    let output = m.join(" ");

    println!("{}", output);
}
