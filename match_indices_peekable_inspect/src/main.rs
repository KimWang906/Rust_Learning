// match_indices, indices = indexes
// peekable : .peek(), .next()
// .inspect() : 생각한 대로 결과가 나오지 않을 경우 사용되는 매서드 

fn user_input() -> String {
// do something
// random string
    "Rule".to_owned()
}

fn main() {
    let rules = "Rule number 1: No fighting.
Rule number 2: Go to bed at 8 pm.
Rule number 3: Wake up at 6 am.";

    // 해당 문자열이 포함된 첫 번째 index를 반환
    let rule_locations: Vec<(_, _)> = rules.match_indices(&user_input()).collect();
    println!("Rule locations: {rule_locations:?}");

    let just_numbers = vec![1, 5, 100];

    let mut number_iter = just_numbers.iter().peekable();

    // anonymous loop
    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        number_iter.next();
    }

    let new_vec = [8, 9, 10];

    // Vec<_> : type 자동추론
    let double_vec: Vec<_> = new_vec
        .iter()
        .inspect(|first_item| {dbg!(first_item);})
        .map(|x| x * 2)
        .inspect(|next_item| {dbg!(next_item);})
        .filter(|num| *num > 17)
        .collect();

    dbg!(double_vec);
}
