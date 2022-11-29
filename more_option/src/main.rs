// Option<T>

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}
// wrap in an Option

fn main() {
    let new_vec = vec![1, 2, 1, 1, 1, 2]; // Some(1)
    let index = take_fifth(new_vec); // Option<i32>

    // println!("index number: {}", index.unwrap());
    // .unwrap() - take out what is inside

    // Panic이 발생되거나 오류가 생긴 상황에서 사용하는 매서드, 오류 메세지를 정의할 수 있다.
    index.expect("Needed at least five items - make sure Vec has at least five");

    match index {
        Some(number) => println!("I got a number: {}", number),
        None => println!("There was nothing inside"),
    }

    // Some(number)
    // bool
    if index.is_some() {
        // Option<i32>
        // if문에서 some인지 검사를 하고 오기에 unwrap을 해도 안전하다.
        println!("I got a number: {}", index.unwrap());
    }
}
