#![allow(unused_variables)]
#![allow(dead_code)]
use std::cell::Cell;
// interior mutability
// changing on the inside
// & immutable reference / shared reference
// &mut mutable reference / unique reference
// Cell - small room(mut 없이 값을 수정할 수 있다.)
// RefCell
// Mutex
// RwLock

// Cell에 대한 Thread 관련 예시
// not thread safe
// let mut x = 9; // 11
// thread 1 {
//    x += 1;
// }
// thread 2 {
//    x += 1;
// }

// Cell<T>
#[derive(Debug)] // 이 매크로가 Debug를 구현한다.
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>
}

fn main() {
    // Cell::new()를 하여 값을 집어넣는다. 
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_owned(),
        model_name: "Super Phone 3000".to_owned(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true)
    };

    let my_cell = Cell::new(String::from("I am a String"));
    my_cell.set(String::from("I am String!?!??!!???!"));
    // .into_inner()은 .unwarp()과 비슷함
    // let my_string = my_cell.get();

    println!("{super_phone_3000:?}");
    super_phone_3000.on_sale.set(false);
}
