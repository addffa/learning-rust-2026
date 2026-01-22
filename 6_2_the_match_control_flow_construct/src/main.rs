#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("us state: {state:?}");
            25
        },
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(num) => Some(num + 1),
        None => None,
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("1 penny = {} cents", value_in_cents(penny));

    let quarter = Coin::Quarter(UsState::Alaska);
    println!("1 quarter = {} cents", value_in_cents(quarter));

    let num = Some(7);
    println!("num + 1 = {:?}", plus_one(num));

    let dice = 7;
    match dice {
        1 => println!("lucky 1!"),
        9 => println!("tail 9!"),
        num => println!("{num} between 1 and 9!"),
        // or use placeholder _ if we don't want to use the variable
    }
}
