#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // implementation block, associated with the rect type
    fn area(&self) -> u32 {
        // &self in parameter is short for self: &self, we just want to read the data not write, if
        // not it'll be &mut self
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
