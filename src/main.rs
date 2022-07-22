#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

#[derive(Debug)]
struct Human {
    age: u8,
    human_type: HumanType
}

#[derive(Debug)]
enum HumanType {
    He,
    She
}

//impl는 구조체(struct, enum) 안에서의 함수를 만듭니다.
impl Animal {
    fn new_cat(age: u8) -> Self {
        //Self는 해당 구조체(Animal)를 가리킵니다.
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog
        }
    }

    fn print(&self) {
        println!("The am a: {:?}", self);
        // {:?} --> Debug print
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am: {:?}", self);
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat! Now I am: {:?}", self);
    }

    fn change_to_age(&mut self) {
        self.age += 1;
        println!("Changed to age! Now I am: {:?}", self);
    }
}

impl Human {
    fn new_man(age: u8) -> Self {
        Self {
            age,
            human_type: HumanType::He
        }
    }

    fn new_women(age: u8) -> Self {
        Self {
            age,
            human_type: HumanType::She
        }
    }

    fn print(&self) {
        println!("The am a: {:?}", self);
        // {:?} --> Debug print
    }

    fn change_to_man(&mut self) {
        self.human_type = HumanType::He;
        println!("Changed to man! Now I am: {:?}", self);
    }

    fn change_to_women(&mut self) {
        self.human_type = HumanType::She;
        println!("Changed to women! Now I am: {:?}", self);
    }

    fn change_to_age(&mut self) {
        self.age += 1;
        println!("Changed to age! Now I am: {:?}", self);
    }
}

fn main() {
    // var my_animal을 만들어 Animal::new_cat에 10의 값을 줍니다.
    let mut my_animal = Animal::new_cat(10);
    let mut human = Human::new_man(17);

    my_animal.print();
    human.print();
    /* 
        현재 self가 struct Animal을 가리키고 있으므로 
        my_animal에 지정되어 있는 값을 바탕으로 출력합니다. 

        Syntactic Sugar:
        프로그래밍 언어 차원에서 제공되는 논리적으로 간결하게 표현하는 것이고,
        중복되는 로직을 간결하게 표현하기 위해 나타나게 되었습니다.
        ex ) 
        Animal::print(&my_animal); 중복됨
        my_animal.print(); Syntactic Sugar가 들어간 코드
    */
    my_animal.change_to_dog();
    my_animal.change_to_cat();
    my_animal.change_to_age();

    println!();
    
    human.change_to_man();
    human.change_to_women();
    human.change_to_age();
}