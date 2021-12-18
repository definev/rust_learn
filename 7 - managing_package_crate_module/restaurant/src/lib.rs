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

mod back_to

// -------------------------