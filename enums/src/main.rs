use std::net::IpAddr;

enum Option<T> { // rust does not have nulls but enum that can encode the concept of a value being
    // present or absent
    // T is a generic type parameter
    None,
    Some(T), // We have to convert Option<T> to a T before we can perform T operations
    // In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. 
}
let some_number = Some(5); // some kinda value
let some_char = Some('e');
let absent_number: Option<i32> = None; // absence of value

let x: i8 = 5;  
let y: Option<i8> = Some(5); // rust doesn't understand how to add an i8 and Option<i8> as they are
// different types
let sum = x + y;

enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit, // not data
    Move { x: i32, y:i32 }, //name fields, like a struct
    Write(String), // single String
    ChangeColor(i32, i32, i32), // i32 tuples
}
impl Message { // methods on enums
    fn call(&self) {

    }
}
let m = Message::Write(String::from("hello"));
m.call();

//if we used structs, then each one has its own type instead of just one with enums
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32,i32, i32);
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr::V4(127,0,0,1);
let loopback = IpAddr::V6(String::from("::1"));
// let home = IpAddr::V4(String::from("127.0.0.1"));
// let loopback = IpAddr::V6(String::from("::1")); // put the data directly into the enum variant
// instead of enum inside of a struct
// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// }
// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
fn route(ip_kind: IpAddrKind) {}
