use std::collections::HashMap;

fn cities() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hash_map = HashMap::new();

    for city in canadian_cities {
        city_hash_map.insert(city, "Canada");
    }

    for city in german_cities {
        city_hash_map.insert(city, "Germany");
    }

    println!("{:?}", city_hash_map["Bielefeld"]); // &str
    println!("{:?}", city_hash_map.get("Bielefeld")); // Option<&&str>

    println!("{:?}", city_hash_map["Bad Doberan"]); // &str
    println!("{:?}", city_hash_map.get("Bad Doberan")); // Option<&&str>

    println!("{:?}", city_hash_map["Karlsruhe"]); // &str
    println!("{:?}", city_hash_map.get("Karlsruhe")); // Option<&&str>
}

fn book() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    // book_hashmap.insert(1, "Le Petit Prience");
    // book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    // book_hashmap.insert(1, "Eye of the world");

    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Alrerdy got a book");
    }else {
        book_hashmap.insert(1, "Le Petit Prience");
    }

    // k: &Q
    println!("{:?}", book_hashmap.get(&1));
}

fn main() { // Key -> Value
    println!("Cities : \n");
    cities();
    println!("\nBook :\n");
    book();
}
