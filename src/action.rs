use crate::id_map::Id;
use crate::map::Cell;
use crate::skill::Skill;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionKind {
    Movement,
    Skill(Id<Skill>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    kind: ActionKind,
    target: Id<Cell>,
}
