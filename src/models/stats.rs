use super::class::Class;
// Defines the different character stats in the game
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub health: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
}

pub trait BaseStats {
    fn base_stats(&self) -> Stats;
}

impl BaseStats for Class {
    fn base_stats(&self) -> Stats {
        match self {
            Class::Warrior => Stats {
                health: 70,
                strength: 15,
                intelligence: 5,
                dexterity: 10,
            },
            Class::Mage => Stats {
                health: 65,
                strength: 5,
                intelligence: 15,
                dexterity: 10,
            },
        }
    }
}