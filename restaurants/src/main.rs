use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
