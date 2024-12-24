pub enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Trap,
}

pub struct InstantData {
    spell_types: Vec<SpellType>,
}

pub struct SorceryData {
    spell_types: Vec<SpellType>,
}
