use crate::game_definition::effect::{EffectId, Range};
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type SkillId = Id<Skill>;
pub type SkillMap = Map<Skill>;
pub type ClassMapBuilder = MapBuilder<Skill>;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    name: String,

    cost: i32,
    range: Range,
    precision: Option<f32>,
    effects: HashSet<EffectId>,
}
