use crate::game_definition::map::CellId;
use crate::game_definition::skill::SkillId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionKind {
    Movement,
    Skill(SkillId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    kind: ActionKind,
    target: CellId,
}