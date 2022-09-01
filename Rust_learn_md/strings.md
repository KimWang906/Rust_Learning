# String(문자열)

<p>
str은 보통 <b>&str</b>로 많이 사용합니다.</br>
<b>String(Vector를 이용하여 만들어졌고 메모리 Heap에서 주로 사용)</b>은 <b>Sized type</b>으로 어느정도 크기인지 알 수 있습니다.</br>
<b>&str</b>은 문자열의 크기를 알아야 하는데 기본적으로는 알 수 없어</br>
앞에 붙는 <b>&(주소 연산자)</b>를 통해 데이터의 위치와 크기를 알 수 있도록 명시해주기 위해 필요한 것입니다.</br>
<b>String</b>과 <b>&str</b>의 가장 큰 차이점은 <b>String</b>은 문자열 수정이 가능하지만 <b>&str</b>은 불가능하다는 점입니다.</br>
<b>&str</b>은 보통 문자열 리터럴이나 문자열 슬라이스를 저장하는데 사용됩니다.
</p>

    let my_name01 = "Daniel"; //&str
    let my_name02 = "KimWang09".to_string(); 
    //String을 호출하지 않고도 함수를 사용 가능한 이유는 KimWang09"&str.to_string(); 형식으로 되어있기 때문입니다.
    let my_name03 = String::from("KimWang0906"); 
    //String을 호출하고 그 안에 있는 함수 from()을 사용합니다.
    let mut my_other_name = "KimWang0550".to_string();
    //String형이고 예약어 mut로 인해 값이 변할 수 있습니다.
    my_other_name.push('!');

    println!("{}", my_name01);
    //&str형이고 값을 변경할 수 없습니다.
    println!("{}", my_name02);
    //String형이고 값을 변경할 수 없습니다.
    println!("{}", my_name03);
    //String형이고 값을 변경할 수 없습니다.
    println!("{}", my_other_name);
    //String형이고 push()함수로 '!' 문자를 추가합니다.

## 문자열 매서드

<p align="center"><img src="/images/Vector_Space.png">

[.capacity](https://yonmy.com/archives/43)
</p>
  : 벡터가 보유할 수 있는 요소의 수를 반환합니다.(메모리 재할당 없음)</br>
.push</br>
</br>
  : 문자를 추가할 수 있습니다.(char)</br>
.push_str</br>
</br>
  : 문자열을 추가할 수 있습니다.(String)</br>
.pop</br>
</br>
  : vector의 마지막 값을 꺼내서 Some(value)를 반환하고</br>
    만약 vector가 비었을 경우 None을 반환합니다.</br>
with_capacity</br>
</br>
  : 문자열의 길이를 지정합니다.
</p>

## From, Into

```rs
    let my_name = String::from("Daniel Hyunbin");
    let my_city: String = "Ulsan".into();
```
