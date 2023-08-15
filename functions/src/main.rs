fn main() {
    println!("Hello, world!");
    let y = another_function(5);
    println!("{y}s across the board!");
}

fn another_function(x: i32) -> i32 {
    x + 5
}
