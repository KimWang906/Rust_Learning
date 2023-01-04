// Iteractor = a collection of things that you can call .next() on
// for keyword를 사용할 때도 iteractor가 사용되었다.

// .iter() - iteractor of references : &T
// .iter_mut() - iteractor of mutable references &mut : T
// .into_iter() - consuming iteractor

fn main() {
    let vector1 = vec![1, 2, 3]; // vector1은 vector1, vector2가 iter를 사용하여 소멸되었다.
    // .map() - iteractor로 수집한 원소를 마음대로 수정할 수 있다.
    // .map(|(변수 이름)| 수식)
    // .iter() 매서드로 수집한 원소를 .map()으로 수정한 뒤 .collect()를 이용하여 새로운 Vec에 담는다.
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();

    let mut vector2 = vec![10, 20, 30];
    // .for_each() - iteractor에서 for문을 사용하여 원소가 Reference이기 때문에 DeRef를 하여 값을 수정한다.
    let _vector2_a = vector2.iter_mut().for_each(|num| *num += 100);

    println!("Vector1");
    println!("a : {vector1_a:?}");
    println!("b : {vector1_b:?}");

    println!();

    println!("Vector2 : {vector2:?}");
}
