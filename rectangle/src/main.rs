#[derive(Debug)] // traits
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); // takes ownership, we don't want dbg! to take ownership of the rect1
    println!("rect2 is {:#?}", rect2); // takes a reference
    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    // immutable borrow/reference to the struct so that we can
    // still use rect2 later on in main which is referencing the struct
    rectangle.width * rectangle.height
}
