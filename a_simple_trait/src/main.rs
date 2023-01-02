use std::fmt::{Display, Formatter, Result};

// Display Trait을 구현하였기 때문에 Debug를 사용하지 않아도 된다.
// #[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

// Display를 재정의한다.
impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "My cat name is {name} and it is {age}")
    }
}

// {:?} 없이 {}로 사용 가능
fn main() {
    let mt_mantle = Cat {
        name: "Reggie mantle".to_owned(),
        age: 4
    };

    println!("Mr. Mantle is a {mt_mantle}");
}
