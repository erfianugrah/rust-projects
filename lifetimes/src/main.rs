fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
