use crate::id::*;
use serde::{Deserialize, Serialize};

pub type ClassId = Id<Class>;
pub type ClassMap = Map<Class>;
pub type ClassMapBuilder = MapBuilder<Class>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    name: String,

    // GENERIC STUFF
    health: i32,  
    mana: i32,
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
