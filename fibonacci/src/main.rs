use std::io;

fn main() {
    println!("Enter a fibonacci index to start from: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s: i32 = s.trim().parse().expect("Index entered was not a number");
    println!("Enter a fibonacci index to end at: ");
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("Failed to read line");
    let e: i32 = e.trim().parse().expect("Index entered was not a number");
    // You can put in multiple results: {} and in order you can pass multiple variables in
    for sequence in s..e {
        println!(
            "Index({}), fibonacci number: {}",
            sequence,
            fibonacci(sequence)
        )
    }
    // let sequence = fibonacci(f);
    // println!("Index: {}", sequence);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    // let mut f = [0, 1];
    // let mut index = 0;
    // while index < 100 {
    //     let n = f[index - 1] + f[index - 2];
    //     f.push(n);
    //     println!("n is: {}", f[index]);:
    //     index += 1;
    // }
}
