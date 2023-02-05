#![allow(unused_variables)]
#![allow(dead_code)]

// filter

// filter_map
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_owned()),
        };

        Self {
            name: name.to_owned(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }

    // fn get_name(&self) -> String {
    //     self.name.clone()
    // }
}

fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|month| month.len() < 5)
        // .ins() : 어떤 값을 포함한 경우 bool(true, false)로 반환
        .filter(|monnthh| monnthh.contains("u"))
        .collect();

    let company_vec = vec![
        Company::new("Unbrella Corporation", "Unknown"),
        Company::new("Ovinitiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let all_the_ceos: Vec<_> = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo()) // Some(포함) / None(제외)
        .collect();

    // let filtered_name: Vec<_> = company_vec
    //     .into_iter()
    //     .filter(|name| name.get_name().contains("S"))
    //     .collect();

    println!("{filtered_months:?}");
    println!("{all_the_ceos:?}");
}
