# 문자열 출력 방법

    println!은 기본적인 출력 함수입니다.
    형식은 println!("Hello World!");로 문자열이 출력 가능합니다.
    변수와 함께 출력하고 싶을 경우 println!("{}", 변수) 또는 println!("{변수}")로 사용이 가능합니다. print!(r#"C:\my_drive\programs"#);으로 문자열을 묶게될 경우 이스케이프 시퀀스가 문자로 인식됩니다. 
    println!의 모양이 
    "Hello
    World!"일 경우 공백까지 포함하여 출력하기 때문에 해당 모양 그대로 출력됩니다.
    출력할 때 변수가 들어가는 중괄호에 {:문자}를 넣을 경우 문자의 종류에 따라서 출력이 다르게 나옵니다.
    (:p는 Pointer, :X는 16진수(Hex), :b는 2진수(Byte), :?는 Debug 등 자세한 정보는 https://doc.rust-lang.org/stable/std/fmt/에서 확인이 가능합니다.)
