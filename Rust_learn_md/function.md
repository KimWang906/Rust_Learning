# 함수

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