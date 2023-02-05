#![allow(unused_variables)]
#![allow(unused_assignments)]

// dbg! // debug = quick print


fn main() {

    // file, code line, info
    // [dbg_macro/src/main.rs:6] my_number = 9
    let mut my_number = dbg!(9);

    dbg!(my_number += 10);

    let new_vec = dbg!(vec![8, 9, 10]);

    let double_vec: Vec<i32> = dbg!(new_vec
        .iter()
        .map(|x| x * 2)
        .collect());

}
