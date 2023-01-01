use std::{num::ParseIntError};

// The Question Mark Operator
// ?
// 연산자가 값을 생성해야 하는지 여부를 결정하는 데 사용됩니다

fn parse_str(input: &str) -> Result<(), ParseIntError> {
    let _parsed_number = input.parse::<i32>()?; // return Error
    Ok(())
}

fn main() {
    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
