use crate::enums::atributes::Attributes;

pub enum Status {
    Attribute(Attributes),
    Skills(SkillType),
}

enum SkillType{
    Base(Skill),
    Extra(Skill),
}
impl SkillType {
    fn save(&self) {
        match self {
            SkillType::Base(skill) => {
            }
            SkillType::Extra(skill) => {

            }
        }
    }
}
struct Skill {
    //a skill is something who use a attribute to do something specific
    //when the skill is upgrated, the skill give a mod and bonus on learning
    attribute: Attributes,
    learning: u16,
    experience: u16
}

impl Skill {
    fn get_attribute(&self) -> &Attributes {
        &(self.attribute)
    }
    fn get_mod(&self) -> i32 {
        let attribute = self.get_attribute();
        attribute.get_bonus()
        // add bonus of the bar
    }
}