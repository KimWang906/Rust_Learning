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
