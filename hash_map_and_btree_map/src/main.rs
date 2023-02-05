#![allow(unused_variables)]
#![allow(dead_code)]

// Common Collection Types : Arrays, Vec ...

// Other Collection Types : HashMap, BTreeMap
// Python의 Dictionary와 비슷하다

// Key, Value
// Key: String,
// Value Vec<String>
// land: 나라, 국가

use std::collections::HashMap;
use std::collections::BTreeMap;
// HashMap<String, Vec<String>>

struct City {
    name: String,
    population1: HashMap<u32, u32>, // year + population
    population2: BTreeMap<u32, u32>,
}

fn main() {
    let mut tallin = City {
        name: "Tallinn".to_owned(),
        population1: HashMap::new(),
        population2: BTreeMap::new(),
    };

    // Random
    tallin.population1.insert(1372, 3_250);
    tallin.population1.insert(1851, 24_000);
    tallin.population1.insert(2020, 437_619);

    // Order
    tallin.population2.insert(1372, 3_250);
    tallin.population2.insert(1851, 24_000);
    tallin.population2.insert(2020, 437_619);

    println!("HashMap, population1 : \n");
    // HashMap은 데이터가 랜덤으로 나온다
    // 순서대로 나오는 것은 BTreeMap(정렬된 HashMap)
    for (year, population) in tallin.population1 {
        println!("In the year {} the population was {}", year, population);
    }

    println!("\nBTreeMap, population2 : \n");
    for (year, population) in tallin.population2 {
        println!("In the year {} the popualtion was {}", year, population);
    }
}
