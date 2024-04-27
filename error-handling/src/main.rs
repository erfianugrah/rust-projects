use core::panic;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?; // ? can only be used for types that
                                                             // returns Result, Option etc, if not,
                                                             // Option<T> or Result<T, E> has to be used or match

    Ok(username)
}
fn main() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be include in this project");
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    // panic!("lele panik!"); // add panic ='abort' to toml file if you want to make the binary as,
    // Result<T, E>
    // small as possible
    // let v = vec![1, 2, 3];
    // v[99]; // in C, you might get whatever is at the location that would correspond to that
    //        // element, buffer overread
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
}
