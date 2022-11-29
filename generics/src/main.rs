use std::fmt::Display;

// generics - concrete
// i32 - concrete
// String - concrete

// 아직은 Generics가 없음
// fn print_give_item() -> i32 {
//     let number = 9;
//     println!("The number is: {}", number);
//     9
// }

// struct Book;

// T : type
// <>가 있으면 Rust가 Generic Type인 것을 알 수 있다.
// 웬만한 concrete type에는 Display type이 있으나 아래 generic에는 Display trait이 없음
// 따라서 함수에 Display trait을 추가한다.
fn give_thing<T: Display>(input: T) -> T {
    // 이제 Display trait에 들어올 수 있는 타입만 들어올 수 있다.
    println!("{}", input); // Display Trait이 있어야 사용할 수 있다.
    input
}

fn main() {
    // let _x = print_give_item();
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    // struct Book에 아무 Display trait이 없으므로 에러가 발생한다.
    // Err Code: required by a bound in `give_thing`
    // let z = give_thing(Book);
    println!("{}", x);
    println!("{}", y);
}
