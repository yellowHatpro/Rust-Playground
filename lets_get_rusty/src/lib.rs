fn print_my_name() {
    println!("yellowhatpro");
}

mod front_of_house {
    pub fn print_yellowhatpro() {
        super::print_my_name(); // we can access function in parent module using super
        print_something(); // directly use function within module
    }
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub fn print_something() {
        println!("Yahoo");
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
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
}

//use keyword : allows to bring a path to the scope
pub use self::front_of_house::hosting;
// use crate::front_of_house::hosting; // can also use this, self references to current module

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative paths : start from current module
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    hosting::add_to_waitlist(); // this works using use keyword
}
