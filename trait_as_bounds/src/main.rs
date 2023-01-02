// trait bounds
use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

// Trait Bound로 사용할 것이기 때문에 메서드를 정의하지 않는다.
trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl Magic for Wizard {}
impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}

fn attck_with_bow<T>(character: &T, opponent: &mut Monster, distance: u32)
where
    T: FightFromDistance + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
    }

    println!(
        "You strike with your bow. Your opponent's health now has {} health left. You are now at: {character:?}",
        opponent.health
    );
}

fn attck_with_sward<T>(character: &T, opponent: &mut Monster, distance: u32)
where
    T: FightClose + Debug,
{
    opponent.health -= 10;
    println!(
        "You strike with your sword. Your opponent's health now has {} health left. You are now at: {character:?}",
        opponent.health
    );
}

fn fireball<T>(character: &T, opponent: &mut Monster, distance: u32)
where
    T: Magic + Debug,
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and cast a fireball! Your opponent's health now has {} health left. You are now at: {character:?}",
            opponent.health
        );
    }
}

fn main() {
    let radgast = Wizard { health: 60 };

    let aragorn = Ranger { health: 80 };

    let mut uruk_kai = Monster { health: 40 };

    attck_with_sward(&radgast, &mut uruk_kai, 7);
    attck_with_bow(&aragorn, &mut uruk_kai, 8);
    fireball(&radgast, &mut uruk_kai, 10);
}
