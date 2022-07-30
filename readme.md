# Rust를 써야하는 이유

    C/C++ 언어는 메모리를 프로그래머가 직접 delete합니다.
    모든 메모리를 할당한 역순으로 조금도 남기지 않고 해지해야
    메모리 손실이 없는 프로그램이 됩니다.
    그러나 malloc() 함수로 할당을 하고 free() 함수로 할당을 해제할 때
    포인터가 여전히 애제된 메모리 영역을 가리키고 있을 때에 문제가 발생할 수 있는데
    이를 댕글링 포인터 현상이라고 합니다. 정확한 문제점들은 메모리에 접근할 시 예측 불가능한
    동작을 일으킬 수 있고, 만약 메모리에 접근이 불가할 경우에는
    Segmentation fault가 발생할 수 있으며 잠재적인 보안의 위협이 있습니다.
    또한 자바와 비슷한 언어와 같이 프로그래머는 메모리를 쓰기만 하고 해지하는 것은
    신경을 쓰지 않아도 되는 가비지콜렉션(GC, 쓰레기 수집가) 모델이 있습니다.
    그러나 자바 정도면 범용 언어고 시스템 프로그래밍까지는 아니더라도 
    상당히 오래동안 돌아야 하는 프로그램을 만들어야 합니다.
    또한 성능도 중요한데, 그런 프로그램에서 자바나 파이썬은 메모리 관리의 한계 때문에
    도저히 C++ 수준의 성능을 낼 수가 없습니다.

    Rust는 정면돌파를 선택하였습니다.
    메모리 누수가 일어나지 않게 하고, 쓰레기 수집가를 사용하지 않았습니다.
    필요 없어진 메모리를 조금의 누수도 없이 깨끗하고 빠르게 해지하겠다고 하였습니다.
    프로그래머에게 주는 부담을 최소화하고 컴파일러가 자동으로 해주겠다는
    원대한 꿈을 가지고 시작한 것입니다.

## 유명한 Rust 강의 목록

[mithradates 선생님의 Easy Rust](https://youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE)

## Rust 언어 자습서(E-book)

[Rust-Lang-Book](https://rinthel.github.io/rust-lang-book-ko/)

## 목차

[1. Print](/Rust_learn_md/print.md)

[2. Variables](/Rust_learn_md/variable.md)

[3. Memory & Pointer](/Rust_learn_md/memory%26pointer.md)

[4. Ownership](/Rust_learn_md/ownership.md)

[5. Strings](/Rust_learn_md/strings.md)

[6. Array](/Rust_learn_md/array.md)

[7. Function](/Rust_learn_md/function.md)

[8. Struct](/Rust_learn_md/struct.md)

[9. Loop](/Rust_learn_md/loop.md)

[10. Condition](/Rust_learn_md/condition.md)

[11. Others](/Rust_learn_md/others.md)
