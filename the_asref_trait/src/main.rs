// AsRef : 특정 타입만 허용하고 싶을 때 사용한다.
use std::fmt::Display;

// Bound의 조건 AsRef<str>
fn print_it<T>(input: T) where T: Display + AsRef<str> {
    println!("{input}");
}

fn main() {
    // String, &str로 구현하는 모든 문자열은 사용이 가능하다.
    print_it("To Owned".to_owned());
    print_it("To String".to_string());
    print_it(String::from("String from .."));
}
