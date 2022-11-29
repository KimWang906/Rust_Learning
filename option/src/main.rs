// Option
// Result
// OCaml 언어에서 파생되었다.

// Option은 어떤 것의 값이 있거나 없을 경우, 사용하는 것이 좋다.
// enum Option<T> {
//     None,    // 아무것도 가지고 있지 않음
//     Some(T), // type을 가지고 있음
// }

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        // None과 Some은 기본적으로 Rust가 use std::option::Option::{self, Some, None}을 제공해주기 때문에 선언할 필요가 없다.
        None
    } else {
        // value[4] = i32, Some(value[4]) = Option<i32>
        // wrap in an Option(안전하게 Option 안에 넣고 사용한다.)
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    // panic 발생, 해당 기능은 프로그램 실행을 시도하나 만일 되지 않을 것 같다면 안전하게 프로그램을 종료시킨다.
    let index = take_fifth(new_vec);
    println!("{:?}", index);
}
