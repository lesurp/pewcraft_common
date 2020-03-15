use crate::game_definition::effect::Buff;
use crate::game_definition::class::*;
use crate::game_definition::map::CellId;
use crate::id::*;
use serde::{Deserialize, Serialize};

pub type CharacterId = Id<Character>;
pub type CharacterMap = Map<Character>;
pub type ClassMapBuilder = MapBuilder<Character>;

// TODO: store the applier of the buff's stats here!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuffInstance(Buff, ());


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    name: String,
    class: ClassId,

    current_health: i32,
    current_mana: i32,
    position: CellId,
    buffs: Vec<BuffInstance>,
}
