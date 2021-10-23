// structure of the code
// crate (implicit name for the root)
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order

// use self::back_of_house::Breakfast;
// `pub mod` re-export the module to other callers
pub mod back_of_house;
pub mod front_of_house;

pub use crate::back_of_house::{Appetizer, Breakfast};
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    // front_of_house::hosting::seat_at_table();
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    let mut meal = Breakfast::summer("french");
    meal.toast = String::from("american");
    println!("I'd like a {} toast please", meal.toast);

    let app1 = Appetizer::Soup;
    let app2 = Appetizer::Salad;
    println!("And also {:?} and {:?} please", app1, app2);
}
