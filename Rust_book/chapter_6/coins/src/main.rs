use crate::UsState::Alabama;

enum Coin {
    Penny,
    Nikel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Florida,
}

fn main() {
    let x = value_in_cents(Coin::Quarter(Alabama));
    println!("{x}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            print!("Lucky Penny");
            1
        }
        Coin::Nikel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}