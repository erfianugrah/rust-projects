use std::env; // to able to use the args library, use args_os if unicode
use std::fs; // file system library
fn main() {
    let args: Vec<String> = env::args().collect(); // we call the collect() method on an iterator
                                                   // to turn it into a collection, such as a vector
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file"); // In main, the new statement fs::read_to_string takes the file_path, opens that file, and returns a std::io::Result<String> of the file’s contents.
    println!("With text:\n{contents}");
    // dbg!(args);
}