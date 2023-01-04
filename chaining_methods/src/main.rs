// Chaining methods and functional style
// 계속 매서드를 이어나가는 방식을 Chaining methods and functional style이라고 한다.

fn main() {
    // Chaning methods and functional style #1
    // let my_variable = SomeType {};
    // my_variable
    //     .iter()
    //     .take(8)
    //     .collect()

    let mut new_vec_1 = Vec::new();
    let mut counter = 1;

    while counter < 11 {
        new_vec_1.push(counter);
        counter += 1;
    }
    println!("{new_vec_1:?}");

    let new_vec_2 = (1..=10).collect::<Vec<_>>();
    println!("{new_vec_2:?}");

    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Chaning methods and functional style #2
    // .into_iter() : collection type에 들어있는 원소를 순서대로 하나씩 확인한다.
    // .skip() : n개의 원소를 확인하지 않는다.
    // .take() : n개의 원소를 가져간다.
    // .collect() : 특정 타입의 반복자를 collection 타입으로 변환하고
    //              TurboFish(::<>)를 사용하여 구체적인 collection 타입을 지정할 수 있다.
    let new_vec_3 = my_vec
        .into_iter()
        .skip(3)
        .skip(3)
        .take(4)
        .collect::<Vec<_>>();
    println!("{:?}", new_vec_3);
}
