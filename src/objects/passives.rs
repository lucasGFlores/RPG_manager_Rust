use std::cmp::Ordering;
use crate::objects::character::DiceModification;
use crate::enums::atributes::Attributes;

#[derive(Debug)]
pub struct Passives {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) modification_in_game: Vec<DiceModification>,

}

impl Passives {
    fn get_name(&self) -> &'static str {
        self.name
    }
    fn get_description(&self) -> &'static str {
        self.description
    }
    pub fn get_bonus(&self, skill: &Attributes) -> i32 {
       let list_of_modifications = &self.modification_in_game;
        list_of_modifications.iter().filter_map(|modifications| {
            match modifications {
                DiceModification::DiceMod(list, bonus) if list.contains(skill)  => Some(bonus),
                _others => Some(&0),
            }
        }).sum()
    }
   pub fn get_qtd_of_rolls(&self, skill : &Attributes) -> i8 {
        let list_of_modifications = &self.modification_in_game;
        list_of_modifications.iter().filter_map(|modifications| {
            match modifications {
                DiceModification::RollingMod(list, bonus) if list.contains(skill)  => Some(bonus),
                _others => None,
            }
        }).sum()
    }
    pub fn roll(&self, skill: &mut Attributes, last_dice: &mut u8) {
        for modification in self.modification_in_game.iter() {
            match modification {
                DiceModification::DiceMod(list_of_beneficie_attributes, bonus)
                if list_of_beneficie_attributes.contains(&skill)
                => skill.add_mod_bonus(*bonus),
                DiceModification::RollingMod(list_of_benefice_skills, qtd_of_rools)
                if list_of_benefice_skills.contains(&skill)
                => {
                    skill.get_dice().re_roll(last_dice, qtd_of_rools)
                }
                _ => {}
            }
        }
    }
}
