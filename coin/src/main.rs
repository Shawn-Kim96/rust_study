#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Too bad");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?} state shown", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    let q = Coin::Quarter(UsState::California);
    value_in_cents(q);

    let a = 0u8;
    match a {
        1 => println!("hi"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }


}