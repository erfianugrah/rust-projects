pub mod hosting {
    // both the parent and nested function would need to be public, so it
    // depends on what you're trying to access
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}
