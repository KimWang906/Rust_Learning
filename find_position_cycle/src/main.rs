// find - Option<Self::Item> : 특정 값을 찾아 반환
// position - Option<usize> : 특정 index를 찾아 usize로 반환
// cycle :
// [1, 2, 3].iter().cycle() - 끝나지 않는 iterator

fn main() {
    let num_vec = vec![10, 20, 30, 40 ,50 ,60, 70, 80, 90, 100];

    // 가장 앞에 있는 index를 기준으로 검색
    println!("{:?}", num_vec.iter().find(|&n| n % 3 == 0));
    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));

    // return index
    println!("{:?}", num_vec.iter().position(|&num| num % 3 == 0));
    println!("{:?}", num_vec.iter().position(|&num| num * 2 == 30));

    // even, odd가 계속 반복된다.
    let even_odd = vec!["even", "odd"].into_iter().cycle();

    let even_odd_vec: Vec<(i32, &str)> = (0..6)
        .zip(even_odd)
        .collect();

    println!("{even_odd_vec:?}");
}
