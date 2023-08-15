use std::io;

fn main() {
    // C = 5 / 9 * (F - 32)
    println!("Convert temperature to Celsius:");

    let mut fahrenheit_in = String::new();

    io::stdin()
        .read_line(&mut fahrenheit_in)
        .expect("Failed to read line");

    let fahrenheit: f32 = fahrenheit_in
        .trim()
        .parse()
        .expect("Did not convert to number");

    let celsius: f32 = (5f32 / 9f32) * (fahrenheit - 32f32);

    println!("{fahrenheit} degrees F = {celsius:.2} degrees C");
}
