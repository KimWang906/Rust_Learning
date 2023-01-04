// closure = anonymous functions that capture their environment
// a|nonymous = no name
// enclose

// || - pipes
// () - 일반함수

// example #1
// .iter().map(|item| item + 1).collect()

// example #2
// .iter().map(|item| {
//      let my_number = 1; 
//      item + my_number
//  }).collect()

fn main() {
    let my_number = 10;
    // Closure
    // 다른 함수 내에 있는 변수들을 전부 사용이 가능하다!
    let my_closure = |x: i32| println!("x + my_number : {}", x + my_number);

    let my_closure_block = || {
        let my_number = 7;
        let other_number = 10;
        println!("The two numbers are {my_number} and {other_number}");
        // 리턴도 가능하다
        my_number + other_number
    };

    // 함수처럼 호출 가능
    my_closure(8);
    my_closure_block();
}
