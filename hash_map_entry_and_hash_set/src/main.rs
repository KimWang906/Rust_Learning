use std::collections::HashMap;
use std::collections::HashSet;

// HashMap, HashSet - 순서가 없음
// BTreeMap, BTreeSet - 정렬됨

fn main() {
    // HashMap

    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10)
    ];

    // 빈 HashMap을 생성
    let mut servey_hash = HashMap::new();

    for (gender, number) in data { // (&str, i32)
        // entry() 매서드로 Key를 생성하고 or_insert() 매서드를 통해
        // 비어 있는 경우 기본값(default)을 삽입하여 값이 항목에 있는지 확인하고 항목의 값에 대한 변경 가능한 참조를 반환한 뒤, push를 통해 값을 집어넣는다.
        servey_hash.entry(gender).or_insert(Vec::new()).push(number);
    }

    for (male_or_female, numbers) in servey_hash {
        println!("{:?}, {:?}", male_or_female, numbers);
    }

    // HashSet

    let many_numbers = vec![
        89, 216, 204, 204, 20, 1, 50, 113, 80, 203, 
        55, 135, 116, 107, 3, 75, 192, 40, 103, 135, 
        135, 195, 168, 233, 85, 231, 243, 142, 36, 167, 
        126, 253, 212, 67, 223, 112, 175, 91, 209, 226, 
        246, 179, 164, 108, 211, 160, 84, 200, 114, 3, 
        205, 249, 77, 127, 189, 248, 242, 102, 226, 237, 
        251, 18, 233, 58, 187, 9, 202, 168, 4, 209, 
        18, 80, 80, 93, 196, 148, 157, 148, 209, 125, 
        38, 202, 176, 190, 182, 238, 132, 241, 206, 203, 
        138, 188, 65, 187, 48, 251, 217, 175, 55, 29
    ];

    // 빈 HashSet을 생성
    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number); // ret bool type
    }
    
    let hashset_length = number_hashset.len();
    println!("There are {} unique numbers, so we are missing {}", hashset_length, 100 - hashset_length);

    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{}, ", number);
    }
}
