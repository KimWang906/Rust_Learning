// and
// or

fn main() {
    let one = true;
    let two = false;
    let three = true;
    let four = true;

    println!("\nand : ");
    println!("{}", one && three);
    println!("{}", one && two && three && four);
    println!("\nor : ");
    println!("{}", one || three);
    println!("{}\n", one || two || three || four);

    let first_try = vec![Some("Success!"), None, Some("Success!"), Some("Success!"), None];
    let second_try = vec![None, Some("Success!"), Some("Success!"), Some("Success!"), Some("Success!")];
    let third_try = vec![Some("Success!"), Some("Success!"), Some("Success!"), Some("Success!"), None];

    println!("try(and) : ");

    for index in 0..first_try.len() {
        println!("{:?}", first_try[index].and(second_try[index]).and(third_try[index]));
    }

    println!("\ntry(or) : ");

    for index in 0..first_try.len() {
        println!("{:?}", first_try[index].or(second_try[index]).or(third_try[index]));
    }
}
