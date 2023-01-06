use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
fn serve_order() {}

mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
