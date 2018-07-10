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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 2),
    }
}

fn main() {
    let penny = Coin::Penny;

    let value = value_in_cents(penny);

    println!("{}", value);

    let quarter = Coin::Quarter(UsState::Alaska);

    let other_value = value_in_cents(quarter);

    println!("{}", other_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        Some(v) => println!("{}", v),
        _ => println!("None"),
    }

    match none {
        Some(n) => println!("{}", n),
        _ => println!("None"),
    }

    match none {
        _ => println!("Match everything"),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(n) = some_u8_value {
        println!("{}", n);
    }
}
