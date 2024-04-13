fn main() {
    let number = 3;
    // must always be a boolean, unlike ruby or js, you can't put "if number"
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;
    // if is an expression which will then be used to assign the value to the statement, hence no ;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is {number}");
    // loops();
    nested_loops();
    counter();
    while_loops();
    for_loop();
}
// fn loops() {
//     loop {
//         println!("again!");
//     }
// }
fn counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
fn nested_loops() {
    let mut count = 0;
    // nested loops, break and continue apply to the innermost loop
    // but use a loop label if you want to use break and continue instead of the innermost loop,
    // and it begins with a single quote: '
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn while_loops() {
    let mut number = 3;
    // makes code cleaner since it only runs while the condition is true, removing the need for if else
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}
fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    // to iterate over an array, use a for loop

    for element in a {
        print!("the value is: {element}");
    }
    // .. to input range, and .rev to reverse the count
    for number in (1..4).rev() {
        print!("{number}");
    }
    print!("LIFTOFF!!");
}
