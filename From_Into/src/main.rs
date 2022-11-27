// trait = 초능력
// This type implements (trait name)
//

// From, Into

fn main() {
    // From이 있으면 Into를 사용할 수 있다.
    //
    let my_name = String::from("Dave MacLeod");
    // 만약 Into가 있다는 게 꼭 From이 있는 것은 아니므로
    // 해당 From과 Into를 둘 다 쓰고 싶다면 우선 From을 사용한다.
    // (추후 자세히 알려줄 예정..)
    // From Trait = From<String>, From<&String>, From<&'a String>
    // From Trait을 보면 어떤 타입으로 만들 수 있는지 알 수 있다.
    let my_city: String = "Seoul".into(); // &str

    println!("{}", my_city);

    // Array로 Vec을 만들 수 있다.
    // std::vec::Vec의 From Trait = From<[T; N]> 이 모든 타입으로 Vec을 구현할 수 있다!
    let my_vec = Vec::from([8, 9, 10]); // [i32; 3]
}
