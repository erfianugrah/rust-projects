fn main() {
    // panic!("lele panik!"); // add panic ='abort' to toml file if you want to make the binary as
    // small as possible
    let v = vec![1, 2, 3];
    v[99]; // in C, you might get whatever is at the location that would correspond to that
    // element, buffer overread
}

