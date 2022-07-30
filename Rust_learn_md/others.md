# Rust를 다른 언어와 비슷하게 사용하기(C, C++ 등)

    // uninitialized variable
    // control flow

    //possibly unitialized = maybe doesn't have a value yet;
    //Rust는 자료형과 데이터를 지정해주지 않으면 에러가 생깁니다.

    fn loop_then_return(mut counter: i32) -> i32 { 
        //counter 변수는 예약어 mut로 인해 값이 변할 수 있습니다.
        loop {
            counter += 1;
            if counter % 50 == 0 {
                break;
            }
        }
        counter
    }

    fn  main() {
        let my_number;

        {
            let x = loop_then_return(43); //50
            my_number = x
        };

        println!("{}", my_number);
    }

## read_line으로 입력을 받을 때 \n을 없애는 방법

    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);

    println!("{input} 입력받은 문자열과 추가할 문자열을 이을 수 있다.");

## read_line으로 입력을 받을 때 String형에서 Int형으로 변환하는 방법

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input_number: u32 = input.trim().parse().expect("Please type a number!");

    println!("{}", input_number);

## read_line으로 여러 값을 입력 받을 때(공백)

    let mut input_str = String::new();
    
    // 입력 받은 수를 변수에 담습니다.
    io::stdin().read_line(&mut input_str).expect("Wrong input");

    // 입력 받은 수를 분할한다
    let split_string: Vec<&str> = input_str.split(' ').collect();

    // 분할된 수를 나눠 담습니다.
    let num1 = &split_string[0];
    let num2 = &split_string[1];

    // 나눠 담은 string type의 수 i32로 변환합니다.
    let num1: i32 = num1.trim().parse()
    .expect("Please type a number1!");

    let num2: i32 = num2.trim().parse()
    .expect("Please type a number2!");

    println!("{} {}", num1, num2);

## read_line으로 입력을 받을 때 공백과 \n 동시에 받기

    let mut input_str = String::new();
    let mut input_int = String::new();

    io::stdin().read_line(&mut input_str).expect("Wrong input");

    io::stdin()
    .read_line(&mut input_int)
    .expect("Failed to read line");

    let split_string: Vec<&str> = input_str.split(' ').collect();

    let num1 = &split_string[0];
    let num2 = &split_string[1];

    let num1: i32 = num1.trim().parse()
    .expect("Please type a number1!");

    let num2: i32 = num2.trim().parse()
    .expect("Please type a number2!");

    let num3: i32 = input_int.trim().parse()
    .expect("Please type a number3!");

    println!("{} {} {}", num1, num2, num3);
