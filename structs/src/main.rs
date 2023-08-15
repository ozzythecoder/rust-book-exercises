struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    println!("Hello, world!");

    let user1 = build_user(String::from("Ozzy"), String::from("ozzy@ozzy.com"));

}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}
