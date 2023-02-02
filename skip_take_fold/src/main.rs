// skip, take, fold

use std::cmp::max;

fn main() {
    // ('a'..) : a에서 시작하는 끝나지 않는 Range
    // 'a'에서 시작하는 끝나지 않는 Range에서 10번 take를 한다.
    let ten_chars: Vec<_> = ('a'..).take(10).collect();
    println!("{ten_chars:?}");

    // 1000번 skip 한 후에 10번의 take를 한다.
    let skip_then_ten_chars: Vec<_> = ('a'..).skip(1000).take(10).collect();
    println!("{skip_then_ten_chars:?}");

    let some_numbers = vec![9, 6, 9, 10, 11];

    // init: 0
    // 0, 9(index: 0)
    // 9, 6(index: 1)
    // 15, 9(index: 2)
    // 24, 10(index: 3)
    // 34, 11(index: 4)
    // res: 45
    // 초기값을 설정하고 closure를 통해 원소를 통한 계산이 가능하다.
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number));

    let a_string = "I don't have my dashes in me.";

    // fold() 매서드를 사용할 때 colsure에서 매개변수의 이름을 잘 지어주자
    let dashed = a_string
        .chars() // iterator
        .fold("-".to_owned(), |mut string_so_far, next_char| {
            string_so_far.push(next_char); // return ()
            string_so_far.push('-'); // return ()
            string_so_far
        });

    println!("{dashed}");

    let my_vec = vec![-878, 879879, -98798, 0, 76756];

    // i32::MIN : i32의 최솟값
    let biggest = my_vec // biggest
        .into_iter()
        .fold(i32::MIN, |num1, num2| max(num1, num2));

    println!("Biggest is: {biggest}");
}
