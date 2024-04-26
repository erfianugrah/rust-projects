use std::collections::HashMap;

// let v: Vec<i32> = Vec::new(); // vector, store more than one value in a single data structure
// // let v = vec![1, 2, 3];
// let mut v = Vec::new();
// v.push(5);
// v.push(6);
// v.push(7);
// v.push(8);
// let mut s = String::new();
// let data = "initial contents";
//
// let s = data.to_string();
//
// // the method also works on a literal directly
// let s = "initial contents".to_string();
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.insert(String::from("yellow"), 40);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");
    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{}", s);
    // let s3 = String::from("hello, ");
    // let s4 = String::from("world!");
    // let s5 = s3 + &s4;
    // let s5 = String::from("tic");
    // let s6 = String::from("tac");
    // let s7 = String::from("toe");
    //
    // let s8 = format!("{s5}-{s6}-{s7}"); // fn add(self, s: &str) -> String {} deref coercion to make &String into
    // &str
    // let d = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &d[2];
    // println!("The third element is {third}");
    //
    // let third: Option<&i32> = d.get(2);
    // match third {
    //     Some(Third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    // let mut z = vec![100, 32, 57];
    // for i in &mut z {
    //     *i += 50; // dereference operator *
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("red")),
    //     SpreadsheetCell::Float(10.12),
    // ];
}
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
//
// }
//
//
// https://doc.rust-lang.org/book/ch08-02-strings.html
