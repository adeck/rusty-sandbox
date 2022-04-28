
fn main() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn bus_table() {}
        fn take_payment() {}
    }
}


