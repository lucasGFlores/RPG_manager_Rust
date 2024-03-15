use crate::objects::character::{Dice,DiceModification,Character};
use rand::{random, Rng};
use serde::{Deserialize, Serialize};

mod objects;
use crate::enums::atributes;
use crate::enums::atributes::Attributes;
use crate::enums::atributes::Attributes::{Dextreza, Forca};
use crate::objects::character::Dice::D20;
use crate::objects::passives::Passives;

mod enums;

fn exemplo_funcao(x: i32) -> i32 {
    x * 2
}

fn main() {
    let passiva = Passives {
        name: "teste",
        description: "teste",
        modification_in_game: vec![DiceModification::DiceMod(
            vec![Attributes::Forca(0,D20), Attributes::Constituicao(0, D20)], -10),
                                   DiceModification::RollingMod(vec![Dextreza(0, D20)], 2)]
    };

        let passiva2 = Passives {
            name: "teste3",
            description: "teste",
            modification_in_game: vec![DiceModification::DiceMod(
                vec![Attributes::All(D20)], 5),
                                       DiceModification::RollingMod(vec![Dextreza(0, D20)], 2)]
        };
    let peso = Character::new("fusca", 20, 20, vec![passiva,passiva2]);
    println!("{:?} ", peso.roll_skill(Dextreza(0, D20)));

}
