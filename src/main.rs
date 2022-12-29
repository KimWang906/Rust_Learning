use rand::random;

fn random_data_u8() -> Option<u8> {
    let rand_data = random::<u8>();

    if rand_data >= 0 {
        Some(rand_data)
    } else {
        None
    }
}

fn many_data() {
    let mut count = 0u8;
    let mut enter = 10;

    println!("let many_numbers = vec![");
    print!("    ");

    loop {
        count += 1;

        if let Some(number) = random_data_u8() {
            print!("{}", number);
        }

        if count == 10 {
            break;
        } else {
            print!(", ");
        }

        if count == enter {
            println!("");
            print!("    ");
            enter += 10;
        }
    }

    println!("\n];");
}

fn main() {
    println!();
    many_data();
    println!()
}
