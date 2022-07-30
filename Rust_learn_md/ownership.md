# 소유권(OWNERSHIP)

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