pub mod action;
pub mod character;

use character::{Character, CharacterId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CharacterState {
    NoMovement,
    NoAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentCharacter {
    id: CharacterId,
    state: CharacterState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    characters: HashMap<CharacterId, Character>,
    current_char: CurrentCharacter,
}
