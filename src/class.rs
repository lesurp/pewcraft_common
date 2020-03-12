use crate::id::*;
use serde::{Deserialize, Serialize};

pub type ClassMap = Map<Class>;
pub type ClassMapBuilder = MapBuilder<Class>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Class {
    // GENERIC STUFF
    health: i32,  // health obv
    swiftness: i32, // movement units

    // PHYSICAL STUFF
    strength: i32,  // dmg
    dexterity: i32, // precision
    armor: i32,     // defense

    // MAGICAL STUFF
    intelligence: i32,  // dmg
    concentration: i32, // mana (precision for spell is always 100%)
    willpower: i32,     // spell defense
}
