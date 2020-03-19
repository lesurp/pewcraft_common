use crate::game::{Action, Cell, Class, GameMap, Id, Team};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WireAction(pub Action);

#[derive(Deserialize, Serialize)]
pub struct WireNewCharRequest {
    pub game: String,
    pub name: String,
    pub class: Id<Class>,
    pub team: Id<Team>,
    pub position: Id<Cell>,
}

#[derive(Deserialize, Serialize)]
pub struct WireCreatedGame(pub String);

#[derive(Deserialize, Serialize)]
pub struct WireCreatedChar(pub String);

#[derive(Deserialize, Serialize)]
pub struct WireNewGameRequest {
    pub map: Id<GameMap>,
    pub team_size: usize,
}

