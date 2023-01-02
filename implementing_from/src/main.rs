use std::fmt::Display;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl City {
    // 자기 자신을 반환한다.
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_owned(),
            population: population,
        }
    }
}

// Country::from(vec![City, City ...])
impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

fn print_vec<T>(input: &Vec<T>)
where
    T: Display,
{
    for item in input {
        print!("{item} ");
    }
    println!();
}

fn main() {
    let array_vec = Vec::from([8, 9, 10]);
    print_vec(&array_vec);

    // from이 &str과 String을 u8로 변환하여 Vec에 저장한다.
    let str_vec = Vec::from("What kind of vec is this?");
    print_vec(&str_vec);

    let string_vec = Vec::from("What kind of Vec is a String vec?");
    print_vec(&string_vec);

    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    // Vec 형식으로 City를 만든다.
    let finland_cities = vec![helsinki, turku];
    // from으로 Vec<City>를 Country로 만든다.
    // let finland_from = Country::from(finland_cities);
    // from과 같은 결과
    let finland_into: Country = finland_cities.into();

    // finland_from.print_cities();
    finland_into.print_cities();
}
