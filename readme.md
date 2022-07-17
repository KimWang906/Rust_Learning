# Rust를 써야하는 이유

    C/C++ 언어는 메모리를 프로그래머가 직접 delete합니다.
    모든 메모리를 할당한 역순으로 조금도 남기지 않고 해지해야
    메모리 손실이 없는 프로그램이 됩니다.
    그러나 malloc() 함수로 할당을 하고 free() 함수로 할당을 해제할 때
    포인터가 여전히 애제된 메모리 영역을 가리키고 있을 때에 문제가 발생할 수 있는데
    이를 댕글링 포인터 현상이라고 합니다. 정확한 문제점들은 메모리에 접근할 시 예측 불가능한
    동작을 일으킬 수 있고, 만약 메모리에 접근이 불가할 경우에는
    Segmentation fault가 발생할 수 있으며 잠재적인 보안의 위협이 있습니다.
    또한 자바와 비슷한 언어와 같이 프로그래머는 메모리를 쓰기만 하고 해지하는 것은
    신경을 쓰지 않아도 되는 가비지콜렉션(GC, 쓰레기 수집가) 모델이 있습니다.
    그러나 자바 정도면 범용 언어고 시스템 프로그래밍까지는 아니더라도 
    상당히 오래동안 돌아야 하는 프로그램을 만들어야 합니다.
    또한 성능도 중요한데, 그런 프로그램에서 자바나 파이썬은 메모리 관리의 한계 때문에
    도저히 C++ 수준의 성능을 낼 수가 없습니다.

    Rust는 정면돌파를 선택하였습니다.
    메모리 누수가 일어나지 않게 하고, 쓰레기 수집가를 사용하지 않았습니다.
    필요 없어진 메모리를 조금의 누수도 없이 깨끗하고 빠르게 해지하겠다고 하였습니다.
    프로그래머에게 주는 부담을 최소화하고 컴파일러가 자동으로 해주겠다는
    원대한 꿈을 가지고 시작한 것입니다.

### Rust 언어 자습서(E-book)

    https://rinthel.github.io/rust-lang-book-ko/

### 문자열 출력 방법

    println!은 기본적인 출력 함수입니다.
    형식은 println!("Hello World!");로 문자열이 출력 가능합니다.
    변수와 함께 출력하고 싶을 경우 println!("{}", 변수) 또는 println!("{변수}")로 사용이 가능합니다. print!(r#"C:\my_drive\programs"#);으로 문자열을 묶게될 경우 이스케이프 시퀀스가 문자로 인식됩니다. 
    println!의 모양이 
    "Hello
    World!"일 경우 공백까지 포함하여 출력하기 때문에 해당 모양 그대로 출력됩니다.
    출력할 때 변수가 들어가는 중괄호에 {:문자}를 넣을 경우 문자의 종류에 따라서 출력이 다르게 나옵니다.
    (:p는 Pointer, :X는 16진수(Hex), :b는 2진수(Byte), :?는 Debug 등 자세한 정보는 https://doc.rust-lang.org/stable/std/fmt/에서 확인이 가능합니다.)

### 변수

    let은 기본적인 변수 선언입니다.
    예를 들어, let a = 1;을 하면 값이 1인 변수 a가 정의됩니다.
    그러나 let으로 선언된 변수는 대입이 불가능해 값을 변경할 수 없습니다. 
    이를 immutable하다고 합니다.
    
    mut는 변수가 값을 변경할 수 있게 만들어주는 예약어입니다.
    예를 들어, let mut a = 1;을 하면, 값을 변경할 수 있는 변수 a가 정의되고, 그 값은 1이 됩니다.
    이렇게 값을 변경할 수 있는 변수를 mutable하다고 합니다.

    const는 값을 변경할 수 없다는 점에서, let과 비슷합니다.
    그러나 두 가지의 차이점이 있습니다.
    - let과는 달리 선언시 자료형을 지정해주어야 합니다.
    - 선언시 대입하는 값은 상수여야 합니다.
    (함수의 반환값처럼, 런타임 시 달라질 수 있는 동적인 값은 불가능)

    섀도잉(Shadowing)은 마지막에 선언한 변수로 앞서 선언된 변수를 가리는 것을 말합니다.
    let a = 1; let a = 2; let a = 3; 이런 식으로 같은 이름의 변수를 여러 번 정의하면, 
    이후 코드에서 a를 참고할 때 가장 마지막에 선언된 3을 참조합니다.
    예시)
    let country = "대한민국";
    let country_ref = &country;
    let country = 8;

    println!("{} {}", country_ref, country);
    출력되는 값은 대한민국 8입니다.

### const와 static

    const HIGH_SCORE: i32 = 20; 
    const를 사용할 때는 자료형을 정확히 명시해야하고 변수명을 대문자로 쓰는 것이 관례입니다.

    static LOW_SCORE: i32 = 0;
    static에 mut를 사용할 수 있지만 안전성이 낮아집니다.(unsafe)

### 메모리와 참조(포인터) (1)

    let my_number = 15;
    //실제 값 

    let single_reference = &my_number; 
    //single_reference가 my_number의 메모리 주소를 알고 있어 값을 출력할 수 있습니다.
    let double_reference = &single_reference;
    /*  double_reference가 single_reference의 메모리 주소를 알고, 
        single_reference가 my_number의 메모리 주소를 알고 있어 값을 출력할 수 있습니다. 
        (이중 포인터랑 개념이 비슷하다) */
    let five_reference = &&&&&my_number;
    //five_reference가 &&&&&my_number의 메모리 주소를 알고 있어 값을 출력할 수 있습니다.
    //또한 이렇게 작성해도 오류가 발생하지 않는다.
    println!("The first number is {}", single_reference);
    println!("The second number is {}", double_reference);
    println!("The third number is {}", five_reference);

### 메모리와 참조(포인터) (2)

    & : 참조 / * : 역참조
    & : immutable reference / shared reference
    &mut : mutable reference / unique reference
    
    let mut my_number = 9;
    let num_ref = &mut &mut my_number;
    //포인터와 매우 흡사한 개념입니다.
    
    **num_ref = 10;

    println!("Number is now {}", my_number);

### 소유권(OWNERSHIP)

    fn  print_country(country_name: &String) {
        println!("My country is {}", country_name);
    }

    fn main() {
        let country = "대한민국".to_string();
        print_country(&country);
        /* &가 없을 경우 country의 값의 소유권을 해당 함수의 매개변수에게 넘겨줌
           &를 주게될 경우 main함수의 country변수는 소유권을 가지면서 매개변수에게 메모리 주소를 전달합니다. */
        print_country(&country);
        print_country(&country);
        print_country(&country);
    }

### String(문자열)

    /* str은 보통 &str로 많이 사용합니다.
    String(Vector를 이용하여 만들어졌고 메모리 Heap에서 주로 사용)은 Sized type으로 어느정도 크기인지 알 수 있고 
    &str은 문자열의 크기를 알아야 하는데 기본적으로는 알 수 없어
    앞에 &을 통해 데이터의 위치와 크기를 알 수 있도록 명시해주기 위해 필요한 것입니다.
    String과 &str의 가장 큰 차이점은 String은 문자열 수정이 가능하지만 &str은 불가능하다는 점입니다. 
    &str은 보통 문자열 리터럴이나 문자열 슬라이스를 저장하는데 사용됩니다. */
    let my_name01 = "Daniel"; //&str
    let my_name02 = "KimWang09".to_string(); 
    //String을 호출하지 않고도 함수를 사용 가능한 이유는 KimWang09"&str.to_string(); 형식으로 되어있기 때문입니다.
    let my_name03 = String::from("KimWang0906"); 
    //String을 호출하고 그 안에 있는 함수 from()을 사용합니다.
    let mut my_other_name = "KimWang0550".to_string();
    //String형이고 예약어 mut로 인해 값이 변할 수 있습니다.
    my_other_name.push('!');

    println!("{}", my_name01);
    //&str형이고 값을 변경할 수 없습니다.
    println!("{}", my_name02);
    //String형이고 값을 변경할 수 없습니다.
    println!("{}", my_name03);
    //String형이고 값을 변경할 수 없습니다.
    println!("{}", my_other_name);
    //String형이고 push()함수로 '!' 문자를 추가합니다.

### 문자열 매서드

    .capacity
      : 벡터가 보유할 수 있는 요소의 수를 반환합니다.(메모리 재할당 없음)
    .push
      : 문자를 추가할 수 있습니다.(char)
    .push_str
      : 문자열을 추가할 수 있습니다.(String)
    .pop
      : vector의 마지막 값을 꺼내서 Some(value)를 반환하고 
        만약 vector가 비었을 경우 None을 반환합니다.
    with_capacity
      : 문자열의 길이를 지정합니다.

### 배열 - (Collection types)

    //Rust에서 배열은 주로 Buffer를 만들 때 주로 사용됩니다.

    fn main() {
        let array = ["1월", "2월"]; //[&str; 2]
        println!("{:?}", array.get(3));
        //arr[]를 사용하거나 arr.get()을 사용할 수 있고 get()이 비교적으로 안전합니다.
    }

### 배열 - Slices

    fn main() {
        let seasons = ["1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월", "12월"];
        println!("{:?}", &seasons[..]); 
        /*
            [n1..n2]를 할 경우 n1부터 n2 - 1까지 배열이 출력됩니다.
            [n1..=n2]를 할 경우 n1부터 n2까지 배열이 출력됩니다.
            [..](을)를 할 경우 배열 안에 있는 모든 값이 출력됩니다.
            → [n..](을)를 할 경우 n부터 끝까지 값이 출력됩니다.
            → [..n](을)를 할 경우 0부터 n - 1까지 값이 출력됩니다.
            → [..=n](을)를 할 경우 0부터 n까지 값이 출력됩니다.
        */
    }

### 배열 - Vector

    Vec<String>

    초기화 방법
    let my_vec: Vec<String> = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    제일 편한 초기화 방법
    let my_vec = vec![name1, name2];
    push()를 자동으로 해주는 매크로를 사용하여 초기화를 해줍니다.

### 함수 (1)

    //It's trivial to copy the bytes

    //Ownership and copy types
    fn prints_number(number: i32) {
        println!("{}", number);
    }

    //clone types
    fn prints_string(input: String) {
        println!("{}", input);
    }

    //copy - copy types
    //clone - non-copy types

    fn main() {
        let my_number = 8;
        prints_number(my_number);
        prints_number(my_number);

        let my_country = "Korea".to_string();
        prints_string(my_country.clone());
        //Clone(데이터를 복제)을 만들어 데이터가 함수의 매개변수에게 전달됩니다.
        //값이 매개변수에게 전달되면 그 값의 소유권은 해당 매개변수로 지정됩니다.
        prints_string(my_country);
        //값의 소유권이 이미 매개변수에게 있으므로 굳이 사용하지 않아도 됩니다.
    }

### From, Into

    let my_name = String::from("Daniel Hyunbin");
    let my_city: String = "Ulsan".into();

### Tuple

    -Structure-
        let random_tuple = 
        ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);

        println!(
            "Inside the tuple is:

    First item: {:?}
    Second item: {:?}
    Third item: {:?}
    Fourth item: {:?}
    Fifth item: {:?}
    Sixth item: {:?}",
            //Tuple을 출력하는 방법은 배열을 출력하는 방법과 비슷합니다.
            random_tuple.0, // "Here is a name" 
            random_tuple.1, // 8
            random_tuple.2, // 'a'
            random_tuple.3, // 'b'
            random_tuple.4, // [8, 9, 10]
            random_tuple.5, // 7.7
        );

    -Destructuring-
    let str_tuple = ("one", "two", "three"); //Tuple 생성
    let (a, _, _) = str_tuple; // _는 사용하지 않겠다는 뜻입니다.
    배열로도 가능합니다. let str_array = ["one", "two", "three"];
                     let [a, _, _] = str_array;

    println!("Item a is: {}", a); //원하는 값만 출력

### Struct (1)

    //unit struct(이름만 있는 구조체)
    struct FileDirectory;

    //tuple struct
    // 빨리 디버깅을 하고 싶을 때 #[derive(Debug)](추후 이것과 관련된 것에 대해 배울 예정)를 사용합니다.
    struct Color(u8, u8, u8);

    //named struct (기본적인 구조체)
    struct Country {
        populartion: u32,
        capital: String,
        leader_name: String
    }

    fn main() {
        let x = FileDirectory;
        println!("The size is {}", std::mem::size_of_val(&x));

        let my_color = Color(20, 50, 100);
        println!("The second color is {:?}", my_color.1); 

        let canada = Country {
            populartion: 35_000_000,
            capital: "Ottawa".to_string(),
            leader_name: "Justin Trudeau".to_string()
        };

        //variable.method_name
        println!("The population is {}\nThe capital is: {}\nThe leader name is {}", 
        canada.populartion, 
        canada.capital, 
        canada.leader_name);
    }

### Struct (2)

    use std::mem::size_of_val;

    struct Number {
        one: u8,
        two: u8,
        three: u8,
        four: u32
        /*  u8은 1바이트를 차지하고 u32는 4byte를 차지하기 때문에
            one, two, three 변수를 저장할 공간 4byte를 할당하고 four 변수를 저장할 공간 4byte를 
            할당하게 되어 총 8byte가 할당된다. */
    }

    struct Country {
        populartion: u32,
        capital: String,
        leader_name: String,
        all_populartions: [u32; 5500]
        /*  4번째 구조체 변수가 다른 구조체 변수에 비해 너무 크기가 클 경우 
            해당 구조체 변수만 다른 구조체로 옮겨 사용하는 것이 좋을 수도 있다. */
    }

    fn main() {
        let populartion = 35_000_000;
        let capital = "Ottawa".to_string();
        let leader_name = "Justin Trudeau".to_string();

        let my_country = Country { //4번째 구조체 변수로 인해 구조체의 크기가 크게 증가한다.
            //clippy 기능이 있어 구조체 변수의 이름과 같은 변수 이름일 경우 생략 가능하다.
            populartion,
            capital,
            leader_name,
            all_populartions: [500; 5500]
        };

        println!("Country is {} bytes in size", size_of_val(&my_country));
        
        let numbers = Number { //Number 구조체의 크기는 3이다.
            one: 8,
            two: 19,
            three: 20,
            four: 30
        };

        println!("Number Size is: {}", size_of_val(&numbers));
    }

### Enums (1)

    // struct는 선택지 개념이 아닌 해당 구조에 맞게 사용해야 한다.
    // enum은 enum 변수를 선택할 수 있다.

    // enumeration
    enum ThingsInTheSky {
        Sun, // 0
        Stars // 1
    }

    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun, 
                        //ThingsInTheSky를 Sun으로 지정한다.
            _ => ThingsInTheSky::Stars
                        //ThingsInTheSky를 Stars로 지정한다.
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => println!("I can see the sun"),
            //ThingsInTheSky(이)가 Sun일 경우
            ThingsInTheSky::Stars => println!("I can see the starts")
            //ThingsInTheSky(이)가 Stars일 경우
        }
    }

    fn main() {
        let time = 8;
        let sky_state = create_skystate(time);
        check_skystate(&sky_state); 
        //check_skystate에 sky_state의 메모리 주소를 전달하여 확인할 수 있게 합니다.
    }

### Enums (2)

    Ex 1)
    enum Mood {
        Happy,
        Sleepy,
        NotBad,
        Angry
    }

    fn match_mood(mood: &Mood) -> i32 {
        use Mood::*;
        //사용할 경우 enum 변수 앞에 Mood를 호출하지 않아도 됩니다.
        let happiness_level = match mood {
            Happy => 10,
            Sleepy => 6,
            NotBad => 7,
            Angry => 2
        };
        happiness_level //변수를 실행합니다.
    }

    fn main() {
        let my_mood = Mood::NotBad;
        let happiness_level = match_mood(&my_mood);
        println!("Out of 1 to 10, my happiness is {}", happiness_level);
    }

    Ex 2)
    enum Season {
        Spring,
        Summer,
        Autumn,
        Winter
    }

    fn main() {
        use Season::*;
        let four_seasons = vec![Spring, Summer, Autumn, Winter];
        //Vec 형식으로 배열을 저장합니다.
        for season in four_seasons {
            println!("The number is: {}", season as u32);
            //season 변수의 형을 u32으로 변환한 후에 출력합니다.
        }
    }

### Enums (3)

    Ex 1)

    enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGient = 1000,
    DeadStar
    }

    fn main() {
        use Star::*;
        let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGient, DeadStar];
        //Vec! 매크로를 이용하여 백터 형식으로 저장합니다.
        for star in starvec {
            match star as u32 {
                /*  size..에 들어가는 조건문은 컴파일러가 아직 완벽하게 이해하지 못합니다. 
                    (Rust 1.62 Stable Version 기준) */
                size if size <= 80 => println!("Not the biggest star: {}", size),
                size if size >= 80 => println!("Pretty big star: {}", size),
                _ => println!("Some other star")
            };
        }

        println!("What about DeadStar? It is: {}", DeadStar as u32);
    }

    Ex 2)

    enum  Number {
    // 자료형을 효율적으로 사용하고 싶을 때 사용합니다.
    U32(u32),
    I32(i32) // enum 변수에 자료형을 지정합니다.
    }

    fn get_number(input: i32) -> Number {
        // 값이 True, False로 Return될 경우 let var_name으로 선언하는 것을 생략해도 됩니다.
        match input.is_positive() { 
            /* .is_positive를 통해 
            get_number가 양수이면 true를 반환하고 숫자가 0 또는 음수이면 false를 반환합니다. */
            true => Number::U32(input as u32), //자료형 변환
            false => Number::I32(input) //자료형을 처음 선언했던 대로 합니다.
        }
    }

    fn main() {
        use Number::*;
        let my_vec = vec![get_number(-800), get_number(8)];

        for item in my_vec {
            match item {
                // match를 통해 i32에 넣을 수 있는 값인지 u32에 넣을 수 있는 값인지 확인합니다.
                U32(number) => println!("It's a u32 with the value {}", number),
                I32(number) => println!("It's an i32 with the value {}", number),
            }
        }
    }

### Loop

    fn main() {
        let mut counter = 0;
        let mut counter2 = 0;

        println!("Now entering the first loop.");

        'first_loop: loop {
        /*  loop문에 이름을 준 것을 tick이라고 합니다.
            loop문에 이름을 주게 되면 해당 루프문을 지정하여 정지시킬 수 있습니다. */
            counter += 1;
            println!("The counter is now: {}", counter);
            if counter > 9 {
                println!("Now entering the second loop");

                'second_loop: loop {
                    println!("The second counter is: {}", counter2);
                    counter2 += 1;
                    if counter2 == 3 {
                        break 'first_loop;
                        //특정 loop문을 종료할 수 있습니다.
                    }
                }
            }
        }
    }

    fn main() {
        let mut counter = 5;

        let my_number = loop {
            counter += 1;
            if counter % 53 == 3{
                break counter; //break keyword에 return 값을 줄 수 있습니다.
            }
        };

        println!("my number is now: {}", my_number); //value 53
    }

### More Loop

    fn main() {
        let mut counter = 0;

        while counter != 5{
            counter += 1;
            println!("The counter is now: {}", counter);
        }
    }

    fn main() { 
        for number in 0..3 { 
            /*  n..m은 n < m 이고 n..=m은 n <= m으로 생각할 수 있습니다.
                만약 number 변수를 사용하지 않을 경우 _(don't care)를 사용합니다.
                number 변수를 추후 사용할 예정일 경우 _number( _ + 변수 이름 )로 사용합니다. */
            println!("number is {}", number);
        }
    }

### match

    match를 사용하는 이유 : 
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

### Rust를 다른 언어와 비슷하게 사용하기(C, C++ 등)

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

### read_line으로 입력을 받을 때 \n을 없애는 방법

    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);

    println!("{input} 입력받은 문자열과 추가할 문자열을 이을 수 있다.");

### read_line으로 입력을 받을 때 String형에서 Int형으로 변환하는 방법

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input_number: u32 = input.trim().parse().expect("Please type a number!");

    println!("{}", input_number);

### read_line으로 여러 값을 입력 받을 때(공백)

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

### read_line으로 입력을 받을 때 공백과 \n 동시에 받기

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
