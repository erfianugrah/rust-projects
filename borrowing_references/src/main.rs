fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let s2 = String::from("hi");
    test_change(&s2);
    let mut s3 = String::from("hoi"); // to use mutable references, need to also make mutable
                                      // variables
    mut_change(&mut s3);
    let mut s = String::from("hej");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope
    let r2 = &mut s;
    println!("{}", r2);

    let mut z = String::from("apa");

    let g1 = &z;
    let g2 = &z; // this doesn't affect the immutability of the data
    println!("{} and {}", g1, g2);
    // g1 and  g2 goes out of scope here
    let g3 = &mut z;
    println!("{}", g3); // no longer used by g1 and g2, so g3 can work

    let ref_to_null = dangle();
    let ref_to_smth = correct_dangle();
}

fn calc_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is
  // not dropped

// immutable references
fn test_change(some_string: &String) {
    some_string.push_str(", world");
}

fn mut_change(some_string: &mut String) {
    // mutable references can only have one mutable
    // variables, two will cause an error
    some_string.push_str(", world");
}

fn dangle() -> &String {
    // returns a reference to a string
    let s = String::from("hello"); // s is a new string
    &s // return a reference to a String, s
} // s goes out of scope, memory is poof, return it directly not a reference

fn correct_dangle() -> String {
    // returns a reference to a string
    let s = String::from("hello"); // s is a new string
    s // return a reference to a String, s
} // s goes out of scope, memory is poof, return it directly not a reference
