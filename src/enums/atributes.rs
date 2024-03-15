use crate::objects::character::Dice;

#[derive(Debug, Clone)]
pub enum Attributes {
    Forca(i32, Dice),
    Dextreza(i32, Dice),
    Constituicao(i32, Dice),
    Inteligencia(i32, Dice),
    Sabedoria(i32, Dice),
    Carisma(i32, Dice),
    All(Dice),

}

impl Attributes {

    pub fn set_mod_bonus(&mut self, bonus_mod: i32) {
        *self = match self.clone() {
            Attributes::All(dice) =>  Attributes::All(dice.clone()),
            others => Self::new(bonus_mod,others.get_dice().clone(),others.clone())
        }
    }
    pub fn add_mod_bonus(&mut self, bonus_mod: i32) {
        *self = match self.clone() {
            Attributes::All(dice) => Attributes::All(dice.clone()),
            others => Self::new(others.get_bonus() + bonus_mod, others.get_dice().clone(),others.clone()),
        };
    }

    pub fn get_bonus(&self) -> i32 {
        match &self {
            Attributes::Forca(bonus, _) |
            Attributes::Dextreza(bonus, _) |
            Attributes::Constituicao(bonus, _) |
            Attributes::Inteligencia(bonus, _) |
            Attributes::Sabedoria(bonus, _) |
            Attributes::Carisma(bonus, _) => bonus.clone(),
            Attributes::All(_) => 0,
        }
    }
   pub fn get_dice(&self) -> &Dice {
        match self {
            Attributes::Forca(_, dice)
            | Attributes::Dextreza(_, dice)
            | Attributes::Constituicao(_, dice)
            | Attributes::Inteligencia(_, dice)
            | Attributes::Sabedoria(_, dice)
            | Attributes::Carisma(_, dice)
            | Attributes::All(dice) => dice,
        }
    }

    // Método auxiliar para criar uma nova instância de Self
    fn new(stats: i32, dice: Dice,attribute_type : Attributes) -> Self {
        match attribute_type {
            Attributes::Forca(_, _) => Attributes::Forca(stats, dice),
            Attributes::Dextreza(_, _) => Attributes::Dextreza(stats, dice),
            Attributes::Constituicao(_, _) => Attributes::Constituicao(stats, dice),
            Attributes::Inteligencia(_, _) => Attributes::Inteligencia(stats, dice),
            Attributes::Sabedoria(_, _) => Attributes::Sabedoria(stats, dice),
            Attributes::Carisma(_, _) => Attributes::Carisma(stats, dice),
            Attributes::All(_) => Attributes::All(dice),
        }
    }
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Attributes::Forca(_, _), Attributes::Forca(_, _))
            | (Attributes::Dextreza(_, _), Attributes::Dextreza(_, _))
            | (Attributes::Constituicao(_, _), Attributes::Constituicao(_, _))
            | (Attributes::Inteligencia(_, _), Attributes::Inteligencia(_, _))
            | (Attributes::Sabedoria(_, _), Attributes::Sabedoria(_, _))
            | (Attributes::Carisma(_, _), Attributes::Carisma(_, _))
            => true,
            (Attributes::All(_), others) | (others, Attributes::All(_)) => true,
            _ => false,
        }
    }
}