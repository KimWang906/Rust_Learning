#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::RefCell;

// unwind the stack
// panic

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>
    // Many other fields
}


fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true)
    };

    // runtime checked borrowing rules
    // compile time

    println!("{user_1:?}");
    let mut first_reference = user_1.active.borrow_mut();

    println!("{user_1:?}");
    *first_reference = false;

    println!("{user_1:?}");
    drop(first_reference);

    println!("{user_1:?}");
    {
        
    }
}
