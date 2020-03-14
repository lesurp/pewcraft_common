use class::{Class, ClassId};
use map::GameMap;
use serde::{Deserialize, Serialize};
use skill::{Skill, SkillId};
use std::collections::HashMap;

pub mod buff;
pub mod class;
pub mod map;
pub mod skill;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDefinition {
    classes: HashMap<ClassId, Class>,
    skills: HashMap<SkillId, Skill>,
    maps: Vec<GameMap>,

    class_to_skills: HashMap<ClassId, Vec<SkillId>>,
    skill_to_classes: HashMap<SkillId, Vec<ClassId>>,
}
