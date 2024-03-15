use std::cmp::Ordering;
use rand::{random, Rng};
use crate::enums::atributes::Attributes;
use crate::enums::atributes::Attributes::Dextreza;
use crate::objects::character::Dice::D20;
use crate::objects::character::DiceModification::{DiceMod, RollingMod};
use crate::objects::character::Stress::{BrokenDown, Exhausted, Normal};
use crate::objects::passives::Passives;

#[derive(Debug)]
pub struct Character {
    // TODO skills, disvantage,bonus and passive, and use them all
     name : &'static str,
    hp : u16,
    mental_stress: Stress,
    body_stress: Stress,
    atributes: [Attributes;6],// maybe a enum is gonna make more sense
    passives : Vec<Passives>,
}
#[derive(Debug,Clone)]
enum Stress {
    Normal(u16, u16),
    Exhausted(u16, u16),
    BrokenDown(u16)
}
 impl Stress {
     fn add(&self,addictor: u16) -> Stress {
        match &self {
            Normal(max, actual) => {
             if &addictor+actual >= max/2 {
                  return  Exhausted(*max, *actual).add(addictor);
             }
                Normal(*max, addictor+actual)
            }
            Exhausted(max, actual) => {
                if actual+addictor >= *max {
                   return  BrokenDown(*max);
                }
                Exhausted(*max, actual+addictor)
            }
           BrokenDown(max) => self.clone()
        }
     }
     fn decreasae(&self,reducer: u16) -> Stress {
        match &self {
            BrokenDown(max) => {
                    Exhausted(*max, *max).decreasae(reducer)
            }
            Exhausted(max, actual) => {
                if actual-reducer < max/2 {
                    return Normal(*max, *actual).decreasae(reducer);
                }
                Exhausted(*max, actual-reducer)
            }

            Normal(max, actual) => {
                if let Some(result) = actual.checked_sub(reducer){
                    return  Normal(*max, actual - reducer);
                }
                Normal(*max, 0)
            }

        }
     }

 }
#[derive(Debug)]
// pub enum DiceModification {
//     Dice_mod(fn( &mut Atributes) -> ()), //receive  the Atribute to give a Atribute with the bonus
//     Rolling_mod( fn(&Atributes, last_row:u8) -> u8) //stats or skils, fn(dice_enum) -> final number of the roll
// }
// a simple way
pub enum DiceModification{
    DiceMod(Vec<Attributes>, i32), //receive  the Atribute to give a Atribute with the bonus
    // use the status inside of the atribute of the vector as a bonus?
    RollingMod(Vec<Attributes>, i8) //stats or skils and roll with advantage or desvantage
}

impl DiceModification {
    fn new(list : Vec<Attributes>,bonus : i32,dice_type : DiceModification) -> DiceModification{
        match dice_type {
            DiceMod(_, _) => DiceMod(list,bonus),
            RollingMod(_, _) => RollingMod(list,bonus as i8),

        }
    }
   // pub fn add_bonus(&mut self,bonus_add: &i32) {
   //
   //      *self = match self {
   //          DiceMod(list, bonus) => Self::new(list,bonus+bonus_add,DiceMod(vec![],0)),
   //          RollingMod(list, bonus) => Self::new(list,bonus as i32+bonus_add,RollingMod(vec![],0)),
   //      }
   //  }
}
#[derive(Debug, Clone,PartialEq)]
pub enum Dice{
    D4,
    D6,
    D8,
    D10,
    D12,
    D16,
    D20,
}
impl Dice {
    fn roll(&self) -> u8 {
        let mut rng = rand::thread_rng();
        match &self {
            Dice::D4 => rng.gen_range(1..4+1),
            Dice::D6 => rng.gen_range(1..6+1),
            Dice::D8 => rng.gen_range(1..8+1),
            Dice::D10 => rng.gen_range(1..10+1),
            Dice::D12 => rng.gen_range(1..12+1),
            Dice::D16 => rng.gen_range(1..16+1),
            Dice::D20 => rng.gen_range(1..20+1),
        }
    }
    fn max_roll(&self) -> u8 {
        match &self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 =>12 ,
            Dice::D16 => 16,
            D20 =>20,
        }

    }
    fn min_roll(&self) -> u8 {
        1 //I think dont exist any negative dice, rigth?
    }
    pub fn re_roll(&self,   last_roll:&mut u8, qtd_of_rolls: &i8)  {
        match &self {
            _ => {
                match qtd_of_rolls.cmp(&0) {
                    Ordering::Greater
                    if last_roll < &mut self.max_roll()
                    => {
                        //advantage roll
                        for _roll in 1..=*qtd_of_rolls {
                            print!("{}\n",_roll);
                            let  new_roll = self.roll();
                            if *last_roll < new_roll {
                                *last_roll = new_roll;

                            }
                        }
                    }
                    Ordering::Less
                    if last_roll < &mut self.min_roll()
                    => {
                        //disvantage roll
                        for _roll in 1..=(*qtd_of_rolls*-1) {
                            let new_roll = self.roll();
                            if *last_roll >  new_roll {
                                *last_roll = new_roll;
                            }
                        }
                    }
                    _ => {},

                }
            }
        }
    }

}
 impl Character {
    pub fn new(name : &'static str,hp:  u16,stress : u16,passives :Vec<Passives>) -> Character {
        let atributes:[Attributes;6] = [
            Attributes::Forca(3, D20),
            Attributes::Dextreza(3, D20),
            Attributes::Constituicao(3, D20),
            Attributes::Inteligencia(3, D20),
            Attributes::Sabedoria(3, D20),
            Attributes::Carisma(3, D20),
        ];

        Character {
            name,
            hp,
            mental_stress: Stress::Normal(stress, 0),
            body_stress: Stress::Normal(stress, 0),
            atributes,
            passives,
        }

     }
     pub fn add_mental_stress(&mut self, addictor : u16) {
         self.mental_stress = self.mental_stress.add(addictor);
     }
     pub fn decrease_mental_stress(&mut self,reductor : u16) {
         self.mental_stress =
             self.mental_stress.decreasae(reductor);
     }
     pub fn add_body_stress(&mut self, addictor : u16) {

         self.body_stress = self.body_stress.add(addictor);
     }
     pub fn decrease_body_stress(&mut self,reductor : u16) {
         self.body_stress = self.body_stress.decreasae(reductor);
     }
     pub fn get_status(&self, skill_type : &Attributes) -> Option<Attributes>  {
         let stats = self.atributes.clone().into_iter().filter(|attr| attr == skill_type).collect::<Vec<Attributes>>();
           Some(stats.get(0).unwrap().clone())
     }

     pub fn get_stats_bonus(&self,skill_type : &Attributes) -> i32 {
         const REDUTER_OF_THE_SYSTEM: i32 = 3;
         let  skill = self.get_status(skill_type).unwrap();
         let bonus : i32 = self.passives.iter().map(|passives| passives.get_bonus(&skill)).sum();
        bonus-REDUTER_OF_THE_SYSTEM+skill.get_bonus()
     }
     pub fn get_roll_bonus(&self,skill_type : &Attributes) -> i8 {
         let  skill = self.get_status(skill_type).unwrap();
         let roll_bonus = self.passives.iter().clone().map(|passives| passives.get_qtd_of_rolls(&skill)).sum();
         roll_bonus
     }
     pub fn roll_skill(&self,skill_type : Attributes) -> i32 {
         let skill = self.get_status(&skill_type).unwrap();
         let mut roll = skill.get_dice().roll();
         let qtd_of_rolls = self.get_roll_bonus(&skill);
         let bonus = self.get_stats_bonus(&skill);
         if qtd_of_rolls > 0 {
             skill.get_dice().re_roll(&mut roll,&qtd_of_rolls);
         }
         if bonus > 0 {
             print!("Character {}+{}\n", roll, bonus);
         } else {
             print!("Character {}{}\n", roll, bonus);
         }
        roll as i32+bonus
     }

 }

