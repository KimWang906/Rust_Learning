// any - or(bool)
// all - and(bool)

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    // 하나라도 맞다면 true
    println!("Is  {check} inside? {}", char_vec.iter().any(|&character| character == check))
}

fn main() {
    let char_vec: Vec<char> = ('a'..'가').collect();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '깘');

    // short-circuiting
    let smaller_vec: Vec<char> = ('A'..'z').collect();
    // 전부 다 맞다면 true
    println!("All alphabetic? {}", smaller_vec.iter().all(|&character| character.is_alphabetic()));
    println!("All less than the character 행? {}", smaller_vec.iter().all(|&c| c < '행'));
    println!("{smaller_vec:?}");
}
