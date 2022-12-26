fn main() {
    println!("My Vec : \n");

    // let 바인딩 예 : 
    // my_vec이라는 변수에  vec![2, 3, 4]을 만들 수 있다면 만들어주세요
    // 왼쪽(변수)와 오른쪽(만들고자 하는 것)을 패턴 비교를 하여 오른쪽이 가능하다면 만들어라 
    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
        // None에 관심이 없을 때, 다음과 같은 문법(if let)을 사용한다.
        // if와 let 바인딩이 합쳐진 문법
        // let 바인딩 예 2 : Some(number)에 my_vec.get(index)이 가능하다면 만들어주세요. 
        if let Some(number) = my_vec.get(index) {
            println!("{} is number!", number);
        }
        // match my_vec.get(index) {
        //     Some(number) => println!("{} is number!", number),
        //     None => {}
        // }
    }

    println!("\nWeather Vec :\n");

    let weather_vec = vec![
        vec!["Berlan", "cloudy", "5", "-7", "8"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}", city.get(0).unwrap());
        // Some()이 되는 것만 출력
        while let Some(information) = city.pop() {
            // Ok()가 되는 것만 출력
            if let Ok(number) = information.parse::<i32>() { // if let + turbofish(:<>)
                println!("The number is {}", number);
            }
        }
    }    
}
