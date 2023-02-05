#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;

// 'static은 특별한 lifetime이다.
// 'static은 프로그램이 존재하는 내내 살아있을 수 있다.
// Book<'static>은 동작할 수 없다. 
// 그 이유는 static은 프로그램의 시작부터 존재해야 하지만 Book struct는 main() 함수에서 시작하기 때문이다.
// 'static은 static type이 있어야 작동한다. 
// ex) <'static str>

// 어떤 lifetime
struct Book<'a> { // Generics T, U
    name: &'a str,
    second_name:  &'a str
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32
}

// lifetime을 사용한 impl에 아래와 같은 오류가 발생한다.
// error[E0726]: implicit elided lifetime not allowed here
// implicit == 말하지 않음
// elided == 보여주지 않음
// 해결하는 방법은 <'_>(이미 정해진 lifetime)로 컴파일러에게 lifetime 추론이 가능하게끔 한다.
impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

// T: Display가 포함된 변수만 받을 수 있음
fn print_thing<T: Display>(input: T) {

}


fn main() {
    let my_book_title = "My Hero Academia
Midoriya Izuku : Origin".to_string();

    let my_book = Book {
        // name을 lifetime &'static str로 받게될 경우 static type으로 나와야 하는 값이
        // 현재는 borrowed str이므로 아래와 같은 오류가 발생하게 된다.
        // error[E0597]: `my_book_title` does not live long enough
        // name: &my_book_title
        // 따라서 다음과 같이 수정한다.
        // Book의 lifetime을 
        name: &my_book_title,
        second_name: ""
    };
}
