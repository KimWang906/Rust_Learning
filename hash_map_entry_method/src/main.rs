use std::collections::HashMap;

fn main() { // Key -> Value
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prience",
        "섀도우 오브 유어 스마일",
        "Eye of the world",
        "Eye of the world"
    ]; // Eye of the world appears twice

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
    }

    for (book, number) in book_hashmap {
        println!("{} : {} copies", book, number);
    }
}   
