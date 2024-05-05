
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("cents: {}", cents);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // match all
        // _ => reroll(),
        // ignore everything else
        _ => ()
    }


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?} none: {:?}", six, none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn reroll() {}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}