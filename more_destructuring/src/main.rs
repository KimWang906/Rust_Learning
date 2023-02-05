#![allow(unused_variables)]
#![allow(dead_code)]

// destructuring

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8,
}

impl Person2 {
    fn from_person(i: Person) -> Self {
        let Person { name, height, .. } = i;

        // return Self { name, height }
        Self { name, height }
    }
}

fn main() {
    // default struct
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let person2 = Person2::from_person(papa_doc);

    // destructuring
    // let Person {
    //     name,
    //     real_name,
    //     height,
    //     happiness,
    // } = papa_doc;

    // destructuring + 별칭으로 사용하기
    // let Person {
    //     name: n,
    //     real_name: r,
    //     height: h,
    //     happiness: ha,
    // } = papa_doc;

    // 아래와 같이 사용하면 너무 길어지므로
    // println!(
    //     "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
    //     papa_doc.name, papa_doc.real_name, papa_doc.height, papa_doc.happiness
    // );

    // destructuring을 이용한 papadoc을 아래와 같이 사용한다.
    // println!(
    //     "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
    //     name, real_name, height, happiness
    // )

    // destructuring과 별칭을 사용하여 더 간편한 이름을 지을 수 있다.
    // println!(
    //     "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
    //     n, r, h, ha
    // );

    // impl를 이용해 destructuring을 사용하기
    // type is..?
    println!("Person2 type is {:?}", person2);
}
