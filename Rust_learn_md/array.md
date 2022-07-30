# 배열 - (Collection types)

<p>Rust에서 배열은 주로 Buffer를 만들 때 주로 사용됩니다.</p>

    fn main() {
        let array = ["1월", "2월"]; //[&str; 2]
        println!("{:?}", array.get(3));
        //arr[]를 사용하거나 arr.get()을 사용할 수 있고 get()이 비교적으로 안전합니다.
    }

## 배열 - Slices

    fn main() {
        let seasons = ["1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월", "12월"];
        println!("{:?}", &seasons[..]); 
        /*
            [n1..n2]를 할 경우 n1부터 n2 - 1까지 배열이 출력됩니다.
            [n1..=n2]를 할 경우 n1부터 n2까지 배열이 출력됩니다.
            [..](을)를 할 경우 배열 안에 있는 모든 값이 출력됩니다.
            → [n..](을)를 할 경우 n부터 끝까지 값이 출력됩니다.
            → [..n](을)를 할 경우 0부터 n - 1까지 값이 출력됩니다.
            → [..=n](을)를 할 경우 0부터 n까지 값이 출력됩니다.
        */
    }

## 배열 - Vector

    Vec<String>

    초기화 방법
    let my_vec: Vec<String> = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    제일 편한 초기화 방법
    let my_vec = vec![name1, name2];
    push()를 자동으로 해주는 매크로를 사용하여 초기화를 해줍니다.
