fn main() {
    let my_vec = vec!['a', 'b', '아', '히'];
    let mut my_vec_iter = my_vec.iter(); // Iter<char> Type으로 변환된다.

    // Iter 타입에서 사용할 수 있는 next() 매서드를 사용하여 값을 반환한다.
    // assert_eq!()는 좌측과 우측을 비교하여 다를 경우 Panic을 발생시킨다.
    assert_eq!(my_vec_iter.next(), Some(&'a')); // - index 0
    assert_eq!(my_vec_iter.next(), Some(&'b')); // - index 1
    assert_eq!(my_vec_iter.next(), Some(&'아')); // - index 2
    assert_eq!(my_vec_iter.next(), Some(&'히')); // - index 3
    assert_eq!(my_vec_iter.next(), None); // my_vec.iter()의 next()가 index를 벗어났기 때문에 None을 출력한다.
}
