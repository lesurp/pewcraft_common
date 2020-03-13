use crate::id::*;
use crate::game_definition::class::*;
use crate::game_definition::buff::*;
use serde::{Deserialize, Serialize};

pub type CharacterId = Id<Character>;
pub type CharacterMap = Map<Character>;
pub type ClassMapBuilder = MapBuilder<Character>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CharacterState {
    NoMovement,
    NoAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: String,
    class: ClassId,

    current_health: i32,
    current_mana: i32,
    state: CharacterState,
    buffs: Vec<Buff>,
}
