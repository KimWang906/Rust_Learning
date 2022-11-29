use std::cmp::PartialOrd;
use std::fmt::Display;

// std::cmp::PartialOrd trait이 구현되지 않아 오류가 발생한다.
// 따라서 제네릭 타입 U를 통해 Display와 PartialOrd 트레잇을 함께 사용하여 구현해준다.
fn compare_and_print1<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

// 위처럼 제네릭을 명시한 경우와 다르게 keyword where을 추가한 경우도 있다.
fn compare_and_print2<DisplayType, CompareType>(
    statement: DisplayType,
    num_1: CompareType,
    num_2: CompareType,
) where
    DisplayType: Display,
    CompareType: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        // std::cmp::PartialOrd trait이 구현되지 않아 오류가 발생한다.
        // 따라서 제네릭 타입 U를 통해 Display와 PartialOrd 트레잇을 함께 사용하여 구현해준다.
        num_1 > num_2
    );
}

fn main() {
    compare_and_print1("Listen up!", 9, 8);
    compare_and_print2("Listen up!", 9, 8);
}
