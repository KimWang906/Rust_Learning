use std::collections::VecDeque;

// VecDeque
fn main() {
    let mut my_vec = vec![0; 600_000];

    println!("Vec Speed");
    // Vec은 .push() .pop()이 빠르나 .remove()를 맨 앞에 있는 원소에 사용하면 앞으로 전체적으로 이동되며 Vec의 속도가 느려진다.
    for _ in 0..600_000 {
        my_vec.remove(0);
    }

    println!("VecDeque Speed");
    let mut my_vecdeque = VecDeque::from(my_vec);
    for _ in 0..600_000 {
        my_vecdeque.pop_front();
    }
}
