fn main() {
    // Without let, you can't use mut to make it mutable, so you'd have to put mut with let, unless
    // all has let, and this also allows you to shadow variables instead making discrete variables
    let x = 5;

    let x = 6;
    {
        let x = x * 2;
        println!("The value of x in the inner scope: {x}")
    }
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("This is this long: {spaces}");
}
