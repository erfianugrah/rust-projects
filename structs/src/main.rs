fn main() {
    let mut erfi = User {
        // entire field must be mutable
        active: true,
        username: String::from("stoicopa"),
        email: String::from("a@b.com"),
        sign_in_count: 10,
    };
    // let anugrah = User {
    //     active: erfi.active,
    //     username: erfi.username,
    //     email: String::from("d@b.com"),
    //     sign_in_count: erfi.sign_in_count,
    // };
    let anugrah = User {
        email: String::from("d@b.com"), // those fields from erfi can no longer be used because
        // it's moved to anugrah, unless it's integers or bool, then it's copy
        ..erfi
    };

    erfi.email = String::from("c@b.com");
    // build_user(erfi.email, erfi.username);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subj = AlwaysEqual;
}

struct User {
    active: bool,
    username: String, // owned String type instead of &str for string slice, we can use &str but we
    // need to specify lifetimes, how long the value should be valid for
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 10,
    }
}

struct Color(i32, i32, i32); // even though they are of the same type, they cannot take each other
                             // as an argument
struct Point(i32, i32, i32);

struct AlwaysEqual; // unit-like structs without any fields, no need for parentheses, Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
