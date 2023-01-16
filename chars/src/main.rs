// .chars() - iterator of char
// .count() - count number of item in iterator
// .char_indices() == .chars().enumerate()

// .chars() - method
// .chars - field(err 발생)

fn main() {
    let big_string = "Hello there, I am a &str";
    let my_vec = vec![8, 9, 10];
    let binding = big_string.clone();

    println!("\nbig_string has {} characters", big_string.chars().count());

    big_string.chars().for_each(|c| print!("{c} ")); 
    binding.char_indices().for_each(|(index, charrrrr)| {
        println!("At index {index} is the char {charrrrr}");
    });

    // arg를 사용하고 싶지 않을 때 사용, toilet closure라고도 한다.
    my_vec.iter().for_each(|_| {
        println!("We don't care about the number");
    });

}
