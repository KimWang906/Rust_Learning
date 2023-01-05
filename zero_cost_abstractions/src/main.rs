// closure
// anonymous function

// zero cost abstractions
// .iter().map().filter().inspect().collect()

fn main() {
    let my_number = 9;
    let _anonymous_function = || println!("I am a function");
    let _closure = || println!("{my_number}");

    let my_vec = vec![8, 9, 10];
    // 다양한 매서드에 활용이 가능하다.
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(3).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{fourth}");
}