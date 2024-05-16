use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 // accesses the first value in the tuple struct
    }
}
fn main() {
    fn hello(name: &str) {
        println!("Hello, {name}");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    // let b = Box::new(5);
    // println!("b = {}", b);
    // let x = 5;
    // let y = &x;
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y); // you can compare a number to a reference of a number, so a deref is
    //                    // required
}
