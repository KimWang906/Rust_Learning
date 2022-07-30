# Tuple

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

## Struct (1)

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

## Struct (2)

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

## Enums (1)

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

## Enums (2)

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

## Enums (3)

<b>Ex 1)</b>

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

<b>Ex 2)</b>

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

## impl(implementation)

<p><b>impl</b>는 구조체(struct, enum) 안에서의 함수를 만드는 <b>Keyword</b> 입니다.</p>

    #[derive(Debug)]
    struct Animal {
        age: u8,
        animal_type: AnimalType
    }

    #[derive(Debug)]
    enum AnimalType {
        Cat,
        Dog
    }

    impl Animal {
        fn new_cat(age: u8) -> Self {
            //Self는 해당 구조체(Animal)를 가리킵니다.
            Self {
                age,
                animal_type: AnimalType::Cat
            }
        }

        fn new_dog(age: u8) -> Self {
            Self {
                age,
                animal_type: AnimalType::Dog
            }
        }

        fn print(&self) {
            println!("The am a: {:?}", self);
            // {:?} --> Debug print
        }

        fn change_to_dog(&mut self) {
            self.animal_type = AnimalType::Dog;
            println!("Changed to dog! Now I am: {:?}", self);
        }

        fn change_to_cat(&mut self) {
            self.animal_type = AnimalType::Cat;
            println!("Changed to cat! Now I am: {:?}", self);
        }

        fn change_to_age(&mut self) {
            self.age += 1;
            println!("Changed to age! Now I am: {:?}", self);
        }
    }

    fn main() {
        // var my_animal을 만들어 Animal::new_cat에 10의 값을 줍니다.
        let mut my_animal = Animal::new_cat(10);
        let mut human = Human::new_man(17);

        my_animal.print();
        human.print();
        /* 
            현재 self가 struct Animal을 가리키고 있으므로 
            my_animal에 지정되어 있는 값을 바탕으로 출력합니다. 

            Syntactic Sugar:
            프로그래밍 언어 차원에서 제공되는 논리적으로 간결하게 표현하는 것이고,
            중복되는 로직을 간결하게 표현하기 위해 나타나게 되었습니다.
            ex ) 
            Animal::print(&my_animal); 중복됨
            my_animal.print(); Syntactic Sugar가 들어간 코드
        */
        my_animal.change_to_dog();
        my_animal.change_to_cat();
        my_animal.change_to_age();
    }
