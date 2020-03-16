use crate::game_definition::class::*;
use crate::game_definition::effect::Buff;
use crate::game_definition::map::CellId;
use crate::id::*;
use serde::{Deserialize, Serialize};

pub type CharacterId = Id<Character>;
pub type CharacterMap = Map<Character>;
pub type ClassMapBuilder = MapBuilder<Character>;

// TODO: store the applier of the buff's stats here!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuffInstance(Buff, ());

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Evil,
    Eviler,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub class: ClassId,

    pub current_health: i32,
    pub current_mana: i32,
    pub position: CellId,
    pub buffs: Vec<BuffInstance>,
    pub team: Team,
}

impl Character {
    pub fn new<S: Into<String>>(
        id: ClassId,
        position: CellId,
        class: &Class,
        name: S,
        team: Team,
    ) -> Character {
        Character {
            name: name.into(),
            class: id,
            current_health: class.health,
            current_mana: class.mana,
            position,
            buffs: Vec::new(),
            team,
        }
    }

    // TODO: look at current debuffs to return the "effective" stats
    pub fn effective_health(&self, class: &Class) -> i32 {
        class.health
    }

    pub fn effective_mana(&self, class: &Class) -> i32 {
        class.mana
    }

    pub fn effective_swiftness(&self, class: &Class) -> i32 {
        class.swiftness
    }

    // PHYSICAL STUFF
    pub fn effective_strength(&self, class: &Class) -> i32 {
        class.strength
    }

    pub fn effective_dexterity(&self, class: &Class) -> i32 {
        class.dexterity
    }

    pub fn effective_armor(&self, class: &Class) -> i32 {
        class.armor
    }

    // MAGICAL STUFF
    pub fn effective_intelligence(&self, class: &Class) -> i32 {
        class.intelligence
    }

    pub fn effective_concentration(&self, class: &Class) -> i32 {
        class.concentration
    }

    pub fn effective_willpower(&self, class: &Class) -> i32 {
        class.willpower
    }
}
