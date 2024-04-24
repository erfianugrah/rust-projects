mod front_of_house {
    use crate::back_of_house;

    pub mod hosting {  // both the parent and nested function would need to be public, so it
        // depends on what you're trying to access
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed to see or modify the
        // seasonal fruit that comes with the meal'
        meal.seasonal_fruit = String::from("blueberries");
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
        hosting::add_to_waitlist();
}
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // we can choose selectively what field to be public
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }   
    }
    pub enum Appetizer { // the variants will also be public
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super is like .., it allows us to reference an item that we
        // know is in the parent module
    }

    fn cook_order() {}
}
