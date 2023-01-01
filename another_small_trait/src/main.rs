struct Monster {
    health: i32,
}

//
struct Wizard {}

struct Ranger {}

trait FightClose {
    fn attck_with_sward(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You strike with your sword! Your opponent's health now {}",
            opponent.health
        );
    }
    fn attck_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You strike with your fist! Your opponent's health now {}",
            opponent.health
        );
    }
}

// FightClose Trait에 대한 Wizard, Ranger는 구현하지 않음
impl FightClose for Wizard {}
impl FightClose for Ranger {}

// Trait 생성
trait FightFromDistance {
    fn attck_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
        }

        println!(
            "You strike with your fist! Your opponent's health now {}",
            opponent.health
        );
    }
    fn attck_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }

        println!(
            "You strike with your fist! Your opponent's health now {}",
            opponent.health
        );
    }
}

impl FightFromDistance for Wizard {}

impl FightFromDistance for Ranger {}

fn main() {
    let radgast = Wizard {};
    let aragon = Ranger {};
    let mut uruk_kai = Monster { health: 40 };

    radgast.attck_with_sward(&mut uruk_kai);
    aragon.attck_with_bow(&mut uruk_kai, 7);
}
