use crate::class::{Class, ClassId};
use crate::effect::Effect;
use crate::id::Map;
use crate::map::GameMap;
use crate::skill::{Skill, SkillId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDefinition {
    pub classes: Map<Class>,
    pub skills: Map<Skill>,
    pub effects: Map<Effect>,
    pub maps: Map<GameMap>,

    pub class_to_skills: HashMap<ClassId, Vec<SkillId>>,
    pub skill_to_classes: HashMap<SkillId, Vec<ClassId>>,
}
