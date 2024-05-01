use std::env; // to able to use the args library, use args_os if unicode

fn main() {
    let args: Vec<String> = env::args().collect(); // we call the collect() method on an iterator
                                                   // to turn it into a collection, such as a vector
    let query = &args[1]; // first argument is always the program itself
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
    // dbg!(args);
}
