#![allow(unused)]
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // A match that only cares about executing code when the value is Some(3)
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value2 = Some(0u8);
    if let Some(3) = some_u8_value2 {
        println!("three");
    }

    match_coin();
    if_let_coin();

}
fn match_coin(){
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin:: Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn if_let_coin() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
