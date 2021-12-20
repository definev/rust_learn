// ------- EXAMPLE 1 -------

// Every child module in parent is private only child can call parent module

mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Private by default
    mod serving {
        fn take_an_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn make_a_order() {
    // Absolute path
    crate::restaurant::hosting::add_to_waitlist();

    // Relative path
    restaurant::hosting::add_to_waitlist();
}

// -------------------------

// ------- EXAMPLE 2 -------

fn serve_order() {}

mod ship_to_house {
    fn fix_incorrect_order() {
        // With the cook order we can call because it in the same module level
        cook_order();
        // serve_order is in parent module which is "crate" in this case,
        // so we can call it by "super" keyword
        super::serve_order();
    }

    fn cook_order() {}
}

// -------------------------

// ------- EXAMPLE 3 -------

pub mod back_to_home {
    // Struct is public but the field alway private by default
    pub struct Breakfast {
        toast: String,
        pub drink: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                drink: String::from("coffee"),
            }
        }
    }
}

fn serve_a_break_fast() {
    let meal = back_to_home::Breakfast::summer("Beefsteak");
    println!("The drink is {}", meal.drink);

    // let error_meal = back_to_home::Breakfast {
    //     // Compiler happy
    //     drink: String::from("milk"),
    //     // Private field is cannot initialize
    //     toast: String::from("Banh mi"),
    // };
}

// -------------------------

// ------- EXAMPLE 4 -------

pub mod zoo {
    // Not like struct, the field in enum is public when the enum public too
    pub enum Animal {
        Tiger,
        Chiwawa,
        Zebra,
    }
}

fn view_animal() {
    let tiger = zoo::Animal::Tiger;
    let chiwawa = zoo::Animal::Chiwawa;
    let zebra = zoo::Animal::Tiger;
}

// -------------------------

// ------- EXAMPLE 4 -------

mod this_is_so_long_long_module {
    pub mod this_is_long_long_module_too {
        pub struct StructWeWantToUse {
            name: String,
        }

        impl StructWeWantToUse {
            pub fn call() {}
        }
    }
}

// "use" keyword make we can reduce the code length
use this_is_so_long_long_module::this_is_long_long_module_too::StructWeWantToUse;

fn call_function() {
    StructWeWantToUse::call();
}

// -------------------------

// ------- EXAMPLE 5 -------

// Instead of duplicate rand::
use rand::CryptoRng;
use rand::Rng;
use rand::RngCore;

// We can use
use rand::{random, Error};

// Import sub-module
use std::io;
use std::io::stdin;

// Import all public member in sub-module
use std::io::*;

// -------------------------

// ------- EXAMPLE 6 -------

// Use self to call module in file
use self::restaurant::hosting::add_to_waitlist;

// -------------------------

// ------- EXAMPLE 7 -------


// -------------------------