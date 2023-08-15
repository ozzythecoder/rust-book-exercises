fn main() {
    let str = String::from("Hello world!");

    // Passes a reference to str, and receives a slice of the first word
    let s = first_word(&str);

    println!("{}", s);
}

fn first_word(s: &str) -> &str {
    // Converts to an array of bytes
    let bytes = s.as_bytes();

    // Iterates over array of bytes
    // enumerate() returns a tuple containing the index of the current element
    // and a reference to the current element.
    for (i, &item) in bytes.iter().enumerate() {
        // byte literal syntax, looking for a whitespace
        if item == b' ' {
            return &s[..i] // returns a slice of the string from beginning to index i
        }
    };

    &s[..] // returns a slice of the entire string
}
