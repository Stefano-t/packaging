#[allow(dead_code)]
pub struct Breakfast {
    pub toast: String,
    // this field is private
    fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("peaches"),
        }
    }
}

#[derive(Debug)]
pub enum Appetizer {
    // they are public by default
    Soup,
    Salad,
}

#[allow(dead_code)]
fn fix_incorrect_order() {
    cook_order();
    // `super` goes to the parent module in the path
    super::front_of_house::serving::serve_order();
}

#[allow(dead_code)]
fn cook_order() {
    println!("cook order");
}
