fn main() {
    let x = 5;
    takes_ownership(x);
    println!("{}", x);
}

fn takes_ownership(s: i32) {
    println!("{}", s);
}
