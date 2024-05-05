use std::error::Error;
use std::fs; // file system library

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Trait object Box<dyn Error>, function
    // will return a type that implements the Error trait, but we don't have to specify, hence
    // dynamic

    let contents = fs::read_to_string(config.file_path)?; // ? will return the error from the
                                                          // current function for the caller to handle

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file"); // In main, the new statement fs::read_to_string takes the file_path, opens that file, and returns a std::io::Result<String> of the file’s contents.
    println!("With text:\n{contents}");
    //
    Ok(()) // need to wrap () from Result in Ok()
           // dbg!(args);
}
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // error values will always be
        // string literls and have the 'static lifetime
        // fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone(); // Instead of lifetimes, clone() makes the code a bit more
//                                   // simpler
//     let file_path = &args[2].clone();
//
//     Config { query, file_path }
// }
