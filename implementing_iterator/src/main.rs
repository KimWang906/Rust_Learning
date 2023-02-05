#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum LibraryType {
    City,
    _Country,
}

#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_owned());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

// Trait과 Struct를 묶어 재정의
impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book_title) => Some(book_title + " is found!"), // String + &str
            None => None
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the darksward");
    my_library.add_book("Demian - die Geschichte einger Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("나의 히어로 아카데미아");

    // Debug print
    println!("My books : {:?}", my_library.books);

    // Iterator + Library
    for item in my_library {
        println!("{item}");
    }
}
