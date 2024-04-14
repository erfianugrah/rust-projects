fn main() {
    variable_scope();
    string_variables();
    let s = String::from("pasta"); // comes into scope
                                   // let w = s;
    take_ownership(s); // moves into function
                       // let y = s; // no longer valid here
    let x = 5; // comes into scope
               // let i = x;
    makes_copy(x); // moves into function
                   // let z = x; // no longer valid here
    let s1 = gives_ownership(); // gives_ownership will move its return value to s1 which is "yours"
    println!("{}", s1);
    let s2 = String::from("cracker"); // comes into scope
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2); // s2 moves into function, which also moves the return value
                                       // into s3
    println!("{}", s3);
    let s4 = String::from("aglioolio");
    let (s5, len) = calc_len(s4);
    println!("The length of '{}' is {}.", s5, len);
} // s3 goes out of scope and is dropped, s2 was moved, so nothing happens, s1 goes out of scope
  // and is dropped

fn variable_scope() {
    // s is not valid here, scope not declared
    // it can be mutated as shown here:
    let mut s = String::from("Hi");
    s.push_str(", there!");
    println!("{}", s);
    // let s = "hello"; // valid from here
    // do stuff with s
} // scope over

fn string_variables() {
    // integer types are known at compile time, but that doesn't mean we shouldn't prevent this
    // this is the Copy trait
    // u32 ,bool ,f64, char, tuples - (i32, i32) but not (i32, string)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // s1 becomes invalidated, a move rather than a copy, but what if want to copy?
    let s1 = String::from("deez");
    let s2 = s1;
    println!("{}", s2);
    // then we use .clone
    let s3 = String::from("nuts");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and is dropped, memory is freed

fn makes_copy(some_int: i32) {
    // some_int comes into scope
    println!("{}", some_int);
} // some_int goes out of scope, nothing special happens

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function
    // that calls it
    let some_string = String::from("nut"); // comes into scope
    some_string // return variable and moves out to the calling function
}

// This functions takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calc_len(s: String) -> (String, usize) {
    // usize is unsigned 64bit arch integer
    let length = s.len(); // len() returns the length, comes into scope
    (s, length) // s is the value we put later in to the function, and length is what we return
                // with said value as a tuple as shown above
}
