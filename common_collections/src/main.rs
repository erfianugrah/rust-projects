// let v: Vec<i32> = Vec::new(); // vector, store more than one value in a single data structure
// // let v = vec![1, 2, 3];
// let mut v = Vec::new();
// v.push(5);
// v.push(6);
// v.push(7);
// v.push(8);
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(Third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
