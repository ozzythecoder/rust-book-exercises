enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

fn count_coins(coin: Coin) -> u8 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
    count += 1;
    count
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let count = count_coins(coin);
    println!("Final count is: {count}");
}
