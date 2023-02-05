#![allow(unused_variables)]
#![allow(dead_code)]

// String
// &str

// Error: cannot return reference to local variable `my_string`
// 해당 변수는 지역변수로 블록({}) 안에서만 살 수 있기 때문에 반환이 불가하다.
// fn returns_reference<'a>() -> &'a str {
//     let my_string = "Hyunbin".to_string(); // &'static - for the life of the program
    
//     &my_string
// }

use std::fmt::Display;

// 어떤 lifetime
struct Book<'book_lifetime> { // Generics T, U
    name: &'book_lifetime str
}

// 프로그램에 존재하는 데이터
fn returns_reference() -> &'static str {
    // let my_string = "Hyunbin".to_string(); // &'static - for the life of the program
    
    // &my_string
    "Hyunbin" // 프로그램이 존재하는 동안 살아있는 데이터이므로 반환가능
}

// T: Display가 포함된 변수만 받을 수 있음
fn print_thing<T: Display>(input: T) {

}

fn main() {
    // &str 중 String literal
    let my_hero_name = "Deku"; // <-- 프로그램이 존재하는 동안 있는 데이터

    // Borrowed str
    {
        let my_string = "David".to_string(); // &'static - for the life of the program
        // &String이지만 Rust가 자동으로 &str로 변환
        let my_string_ref = &my_string; // &str - reference to something else
    }

    let my_book = Book {
        name: "My Hero Academia"
    };
}
