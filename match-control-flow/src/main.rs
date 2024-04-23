#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
    NewYork,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match executes at the first instance of a match to a pattern
    match coin {
        Coin::Penny => 1, // match can be any value, but if needs boolean
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); // debug syntax to pass in a
                                                         // parameter
            25
        }
    } // don't need curly brackets unless you want to run code
}

fn plus_one(x: Option<i32>) -> Option<i32> { // matches needs to be exhaustive, so there should
    // always be a none catch for Option<T>
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5); // Matches Some(i) and becomes 6, since i + 1
let six = plus_one(five); // Matches six since it's i + 1 from the five
let none = plus_one(None); // Matches none

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // nothing happens
    // _ => reroll(),
    /* other => move_player(other),  */// catch all, we can also use _ if don't want to use the value in
    // the catch-all
}

fn add_fancy_hat() {

}
fn remove_fancy_hat() {

}
/* fn move_player(num_spaces: u8) {} */
fn reroll() {}
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
