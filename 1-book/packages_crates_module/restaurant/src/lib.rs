pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}  
    }
}

// bring paths into scope
// shorter path name
use crate::front_of_house::hosting;



pub fn eat_at_restaurant() {
    // Without use
    // // Absolute path 
    // crate::front_of_house::hosting::add_to_waitlist();
    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // With use
    hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("Whole Grain");
    // change our mind
    meal.toast = String::from("Rye");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    let order3 = back_of_house::Appetizer::AcaiBowl;
}


fn deliver_order() {}
pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("mangoes"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
        AcaiBowl,
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}