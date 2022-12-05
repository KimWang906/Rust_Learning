// Option과 Result의 차이점

// Option - 그곳에 있을 수도, 없을 수도 있다.
// .is_some()
// .is_none()
// <T> : Type
// enum Option<T> {
//     None,
//     Some(T),
// }

// .is_ok()
// .is_err()
// // Result - 될 수도 있고 되지 않을 수도 있다.
// // <T, E> : Type, Error
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// None.unwarp -> panic
// Err.unwarp -> panic

fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    // if check_error(5).is_ok() {
    //     println!("It's okay, guys!");
    // } else {
    //     println!("It's an error, guys!");
    // }

    // match check_error(5) {
    //     Ok(_) => println!("Okay guys"),
    //     Err(_) => println!("It's an error"),
    // }

    // .unwarp()
    // Result의 Ok(T)로 반환될 경우, 반환된 값(T)을 사용합니다.
    // Err()로 반환될 경우, Panic이 발생합니다.

    // unwarp() -> Ok()
    let four: u32 = "help".parse().unwrap();
    // unwarp() -> Panic
    check_error(5).unwrap();

    println!("{}", four);
}
