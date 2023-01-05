// map
// for_each

fn main() {
    let num_vec = vec![8, 9, 10];
    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .map(|number| number * 3)
        .collect::<Vec<i32>>();

    let binding = double_vec.clone();
    let _triple_vec = binding
        .iter()
        .enumerate() // (0, 8), (1, 9), (2, 10)
        .for_each(|tuple| { // loop, return ()
            println!("The number at index {} is {}", tuple.0, tuple.1);
        });

    println!("{double_vec:?}");
}
