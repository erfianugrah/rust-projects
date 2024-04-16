fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); //get value of 5
    println!("Length of word is {}.", word);
    let sec_word = second_word(&s);
    println!("Length of second word is {}", sec_word);
    s.clear(); // empties the string
               // word is not connected to the state of s, since we are borrowing the value of 5,
               // and this can still be used to extract the first word out, but would be a bug because the
               // contents of s have changed since we saved 5 in word
    slices();
    slice_main();
}

fn first_word(s: &String) -> usize {
    // pass in string variable and convert to integer
    let bytes = s.as_bytes(); // convert to an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iter is an iteration function, and enumerate
        // creates a tuple with the index, element
        // i for index, &item for the single byte in the tuple
        if item == b' ' {
            // byte literal syntax to look for space
            return i; // return position
        }
    }
    s.len() // if not return length of string
}

fn second_word(s: &String) -> &str {
    // immutable reference to an immutable string s in fn main()
    // pass in string variable and convert to integer
    let bytes = s.as_bytes(); // convert to an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iter is an iteration function, and enumerate
        // creates a tuple with the index, element
        // i for index, &item for the single byte in the tuple
        if item == b' ' {
            // byte literal syntax to look for space
            return &s[0..i]; // return position
        }
    }
    &s[..] // if not return length of string
}

fn third_word(s: &str) -> &str {
    // immutable reference to an immutable string s in fn main()
    // pass in string variable and convert to integer
    let bytes = s.as_bytes(); // convert to an array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iter is an iteration function, and enumerate
        // creates a tuple with the index, element
        // i for index, &item for the single byte in the tuple
        if item == b' ' {
            // byte literal syntax to look for space
            return &s[0..i]; // return position
        }
    }
    &s[..] // if not return length of string
}

fn slice_main() {
    let my_string = String::from("hello world"); // immutable my_string

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = third_word(&my_string[0..6]);
    let word = third_word(&my_string[..]);

    // `first_word` also works on references to `String`s, which are = to whole slices of `String`s
    let word = third_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = third_word(&my_string_literal[0..6]);
    let word = third_word(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without slice
    let word = third_word(my_string_literal);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn slices() {
    let s = String::from("hello world");
    let len = s.len();
    let hello = &s[0..5]; // same as [..5] or if includes last one [3..] or [3..len] or
                          // [0..len]/[..] for the entire string
    let world = &s[6..11];
}
