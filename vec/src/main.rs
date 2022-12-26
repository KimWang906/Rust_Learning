// [u8; 10];
// Vec<T>
// 0 (처음에는 아무 데이터가 없으므로 0)
// reallocation(Rust가 OS에게 메모리를 달라고 요청, 공간을 동적으로 4개씩 늘린다)

// Vec<String>
// Vec<u8>
// T = some type(어떤 타입)
// generics

fn main() {
    /*
        let my_string = String::new(); // 어떤 타입인지 알 수 있다.
        let mut my_vec = Vec::new(); 어떤 타입인지 알 수 없으므로 에러가 발생한다.
        따라서 다음과 같이 수정한다.
        let mut my_vec: Vec<String> = Vec::new();
    */

    // 정의만 하였을 때는 어떤 타입인지 모르나 push를 해줄 경우 컴파일러가 타입을 알 수 있다.
    let mut my_vec = Vec::new(); // Type은 Vec<String>으로 결정된다.

    // Vec : push - pop

    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let name3 = String::from("KimWang906");
    let name4 = String::from("Hyunbin");
    let name5 = String::from("Deku");
    let name6 = String::from("Izuku Midoriya");

    // push type = String
    // 방을 동적으로 4개씩 늘린다.
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name1);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name2);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name3);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name4);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name5);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name6);
    println!("Space for my_vec: {}", my_vec.capacity());

    println!("My cats are {:?}", my_vec);
}
