// map
// and_then

fn main() {
    let some_output = Some(vec![8, 9, 10]);

    let first = some_output
        .clone()
        .map(|some_vec| some_vec.iter().map(|num| num + 1).collect::<Vec<i32>>());

    // let second = some_output.and_then(|some_vec| match some_vec.len() {
    //     0 => None,
    //     1 => Some(vec![some_vec[0]]),
    //     _ => Some(some_vec),
    // });

    let second = some_output
        .map(|some_vec| match some_vec.len() {
            0 => None,
            1 => Some(vec![some_vec[0]]),
            _ => Some(some_vec),
        })
        .flatten(); // flatten

    println!("{first:?}");
    println!("{second:?}"); // - Option<Option<Vec<i32>>> : Option 속 Option, and_then을 사용하여 Option<Vec<i32>>로 만든다.
}
