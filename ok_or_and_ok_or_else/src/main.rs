// .ok() - Result to Option
// .ok_or() - Option to Result
// .ok_or_else() - Option to Result with closure || (더 자세한 에러 메세지를 띄우고 싶은 경우 사용한다)

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
}

fn main() {
    let user_input = vec![
        "8.9",
        "Nine point nine one",
        "8.0",
        "7.6",
        "eleventy-tweleve",
    ];

    let mut result_vec = vec![];

    let actual_numbers: Vec<f32> = user_input
        .into_iter()
        .filter_map(|input| input.parse().ok()) // Ok(num) / Err(err)
        .collect();

    println!("{actual_numbers:?}");

    let company_vec = vec![
        Company::new("Unbrella Corporation", "Unknown"),
        Company::new("Ovinitiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    company_vec.iter().for_each(|company| {
        result_vec.push(company.get_ceo().ok_or_else(|| {
            let err_message = format!("No ceo found for {}", company.name); // format!() - 문자열을 합칠 때 사용한다.
            println!("Oh no! Error: {err_message}");
        })); // option --> result
    });

    println!("{result_vec:?}");
}
