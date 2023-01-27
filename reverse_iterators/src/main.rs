// reverse iterators
// rev() : A(type iterator) --> reverse
// ex) (0..10).into_iter() --> rev() --> (10..0)

fn main() {
    let mut big_vec = vec![6; 100];
    big_vec.push(5);

    let mut iterater = big_vec.iter().rev();
    // test
    assert_eq!(Some(&5), iterater.next());
    assert_eq!(Some(&6), iterater.next());
    // println!("{:?}", iterater.next()); // index 0
    // println!("{:?}", iterater.next()); // index 1


    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));
}
