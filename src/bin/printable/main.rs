// insert the name of the main package
use packaging::back_of_house::{Appetizer, Breakfast};

pub fn main() {
    let meal = Breakfast::summer("Best");
    let app1 = Appetizer::Soup;
    println!("{} with {:?}", meal.toast, app1);
}
