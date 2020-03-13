use crate::game_definition::buff::*;
use crate::game_definition::class::*;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type SkillId = Id<Skill>;
pub type SkillMap = Map<Skill>;
pub type ClassMapBuilder = MapBuilder<Skill>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    name: String,
    classes: HashSet<ClassId>,

    cost: i32,
    range: Range,
    applicable_buffs: HashSet<BuffId>,
}

impl Skill {
    pub fn allow_usage(&mut self, id: ClassId) {
        self.classes.insert(id);
    }
}
