fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    // statements();
    expressions();
    let x = six();
    println!("The value of x is {x}");
    let x = plus_one(7);
    println!("The value of x is {x}");
}
// functions can be called anywhere as long as they are defined somewhere
fn another_function(x: i32) {
    println!("The value of x is {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
// fn statements() {
//     let x = 6; // statement
//     let x = (let y = 6);
// }
fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
// functions can return values to the code that calls them, no need to name just use -> to declare
// the type, either with `return` but usually implicitly
fn six() -> i32 {
    6
}
// always need to declare the type, x here is a 32bit integer, by passing a parameter x, we now
// made a +1 function to any integer that's passed in
fn plus_one(x: i32) -> i32 {
    x + 1
}
