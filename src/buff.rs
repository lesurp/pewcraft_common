use crate::id::*;
use serde::{Deserialize, Serialize};

pub type BuffId = Id<Buff>;
pub type BuffMap = Map<Buff>;
pub type ClassMapBuilder = MapBuilder<Buff>;

// NOTE: we could generate the "Class" struct and the "Attribute" enum jonitly with e.g. a macro
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Attribute {
    Health,
    Swiftness,
    Strength,
    Dexterity,
    Armor,
    Intelligence,
    Concentration,
    Willpower,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValueKind {
    Perc(f32),
    Flat(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuffKind {
    // *all* skills apply buffs - direct damage are just buffs we apply directly
    Damage(ValueKind),
    // negative values mean healing i.e. HoT
    DoT(ValueKind),
    AttributeF(Attribute, ValueKind),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buff {
    name: String,
    turns_left: i32,
    kind: BuffKind,
}
