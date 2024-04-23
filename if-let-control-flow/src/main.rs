#[derive(Debug)]

let config_max = Some(3u8);
// match config_max {
//     Some(max) => println!("The maximum is configured to be {}", max);
//     _ => (),
// }
if let Some(max) = config_max { // no exhaustive checking in match
    println!("The maximum is configured to be {}", max);
}
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:2}!", state),
    _ => count += 1,
}

// or

if let Coin::Quarter(state) = coin {
    println!{"State quarter froim {:?}!", state};
} else {
    count += 1;
}

fn main() {
    println!("Hello, world!");
}
