use minigrep::Config;
use std::env; // to able to use the args library, use args_os if unicode
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect(); // we call the collect() method on an iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // Result<T, E>, if result is an Ok
        // value, it'll be similar to unwrap(), if err, it calls the code in the closure
        eprintln!("Problem parsing arguments: {err}"); // stderr
        process::exit(1);
    });
    /* let config = parse_config(&args);  */// to turn it into a collection, such as a vector
    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
