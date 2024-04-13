use std::io;

fn main() {
    println!("Enter temperature in fahrenheit:");
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to read line");
    let f: i32 = f
        .trim()
        .parse()
        .expect("Temperature entered was not a number");
    let celsius = fahrenheit_celsius(f);
    println!("The current temperature in celsius is: {}", celsius);
}
fn fahrenheit_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}
