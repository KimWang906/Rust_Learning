# Loop

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

## Break문을 활용한 Loop문

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

## More Loop

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
