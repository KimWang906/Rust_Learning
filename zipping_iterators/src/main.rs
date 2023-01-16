// .zip
use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4 , 5];
    let some_words = vec!["zero", "one", "two", "three", "four"];

    // 서로 다른 타입을 HashMap을 통해 묶을 수 있다.
    // 주의 : 두 변수 모두 iterato이면 .zip이 가능하며 두 변수의 길이가 다를 경우 대응되는 값까지만 만들어지고 이외의 값은 제외된다.
    let number_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();

    // unwarp_or_else : Some일 경우 unwarp, None일 경우 closure 내부 코드 실행
    // let result_str = number_hashmap.get(&1).unwrap_or_else(|| {
    //     println!("Help");
    //     &"no number.."
    // });

    number_hashmap.iter().for_each(|stuff| {
        println!("{stuff:?}");
    });
}
