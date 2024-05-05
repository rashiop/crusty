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
