// references and the dot(.) operator
// https://doc.rust-lang.org/nomicon/dot-operator.html

struct Item {
    number: u8,
}

// . dot operator
impl Item {
    fn compare_number(&self, other_number: u8) {
        // *self로 쓰지 않아도 된다.
        println!("Are they equal? {}", self.number == other_number)
    }
}

fn main() {
    let my_number = 10; // i32
    let _reference = &my_number; // &i32

    // {integer} == &{integer}를 비교하고 있으므로 아래 출력문은 오류가 발생한다.
    // println!("Are they the same? {}", my_number == reference);

    // 따라서 다음과 같이 변경
    // println!("Are they the same? {}", my_number == *reference);

    let item = Item { number: 10 };
    // let reference_item1 = &item.number; // &u8
    let referencce_item2 = &item;
    let other_reference_item = &referencce_item2;

    // &u8 == u8을 비교하고 있으므로 아래 출력문에는 오류가 발생
    // println!("{}", reference == 10u8);

    item.compare_number(10);
    /*
        .(dot operator)를 사용하면 Rust가 자동적으로 Ref & 또는 Deref *를 추가해
        type이 일치할 때 까지 Ref & 또는 Deref *를 시도한다.
    */
    referencce_item2.compare_number(10); // &Item
    other_reference_item.compare_number(10);
}
