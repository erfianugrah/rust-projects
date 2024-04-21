#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//
// impl Rectangle {
//     // implementation block, associated with the rect type
//     fn area(&self) -> u32 {
//         // &self in parameter is short for self: &self, we just want to read the data not write, if
//         // not it'll be &mut self
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
impl Rectangle {
    // we give the method the same name as the field only if we want to return the value in the
    // field and nothing else, they are called getters, you can make the field private but the
    // method public
    fn width(&self) -> bool {
        self.width > 0 // this is &self input parameter
    }
    // implementation block, associated with the rect type
    fn area(&self) -> u32 {
        // &self in parameter is short for self: &self, we just want to read the data not write, if
        // not it'll be &mut self
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        // whether the width and height of self are
        // greater than the width and height of the other Rectangle, respectively
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // if rect1.width() {
    //     // automatic referencing, the type of self
    //     // this is the width function
    //     println!("The rectangle has a non-zero width; it is {}", rect1.width); // and this is the
    //                                                                             // width parameter from rect1
    // }
}
