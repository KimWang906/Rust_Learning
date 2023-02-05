use std::cell::RefCell;

fn main() {
    let my_cell = RefCell::new(String::from("I am a String"));
    println!("{my_cell:?}");
    // let reference = my_cell.borrow_mut();
    *my_cell.borrow_mut() = String::from("I am not a String"); // RefMut type
    println!("{my_cell:?}");
}
