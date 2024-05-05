pub mod customer_experience {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}  
        }
    }



    pub fn eat_at_restaurant() {
        // Absolute path 
        crate::customer_experience::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}
pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}