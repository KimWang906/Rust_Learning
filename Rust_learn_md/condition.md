# match

## match를 사용하는 이유

Rust에도 조건문은 존재하지만 조건문이 너무 길어질 경우에는
여러 이유로 좋지 않기 때문에 match를 사용합니다.

    ex) 
    -정수를 이용하여 match를 사용할 경우-
    match my_number { //변수가 하나일 경우에는 ()를 사용하지 않습니다.
        0 => println!("It's a zero"), //조건 1
        1 => println!("It's a one"), //조건 2
        _ => println!("It's a different number"), //모든 조건이 맞지 않을 경우
    };

    변수와 match를 함께 사용이 가능합니다.
    let my_number: u8 = 5;

    let second_number = match my_number { 
        0 => 23, //조건 1
        1 => 65, //조건 2
        _ => 0, // _ : 모든 조건이 맞지 않을 경우
    };

    println!("The second number is: {}", second_number);

    -문자열을 이용하여 match를 사용할 경우-
    let sky = "cloudy"; // &str
    let temperature = "warm";

    /*  match는 조건(패턴)이 맞지 않으면 무조건 다음 조건으로 넘어가기 때문에 
        그 전에 있던 조건을 다시 검사하지 않습니다.
        따라서 _ 문자를 첫 조건으로 사용한다면 컴파일러에서 문제를 확인합니다. */

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"), //조건 1
        ("clear", "warm") => println!("It's a nice day"), //조건 2
        ("cloudy", _) => println!("Cloudy and something else"), //조건 3
        // 여기서 사용하는 _는 어떤 값이 와도 괜찮다는 뜻입니다.
        _ => println!("Not sure what the weather is."), //모든 조건이 맞지 않을 경우
    };

    -if문과 match를 함께 사용하는 경우-
    let children = 5; //i32
    let married = true; //bool

    match (children, married) {
        (children, married) 
        if married == false => println!("Not married with {} children", children),
        //if문을 사용하여 좀 더 정밀한 조건을 사용합니다.
        (c, m) //변수가 위에 고정되어 있기 때문에 match 안에서는 변수 이름을 변경해도 괜찮습니다.
        if  c == 0 && m => println!("Married but with no children"),
        _ => println!("Some other type of marrige and children combination")
    };
