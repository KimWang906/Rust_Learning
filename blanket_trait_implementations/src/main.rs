// blanket_trait_implementations
// implementing a trait for every type that you want to have it

use std::fmt::{Debug, Display};

// blanket trait
trait Prints {
    fn debug_print(&self)
    where
        Self: Debug,
    {
        println!("I am {:?}", self);
    }

    fn display_print(&self)
    where
        Self: Display,
    {
        println!("I am {}", self);
    }
}

#[derive(Debug)]
struct Person;

#[derive(Debug)]
struct Building;

// blanket trait implementations
// 어떤 타입이든 사용이 가능하다.
// Prints에서 타입 T를 명시해주었기 때문에 T가 무엇인지 적지 않아도 알 수 있다.
impl<T> Prints for T {}

fn main() {
    let my_person = Person;
    let my_building = Building;

    my_person.debug_print();
    my_building.debug_print();

    let my_string = String::from("hello");
    my_string.debug_print();
    my_string.display_print();
}
