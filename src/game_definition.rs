use crate::class::Class;
use crate::effect::Effect;
use crate::id_map::{Id, IdMap};
use crate::map::GameMap;
use crate::skill::Skill;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDefinition {
    pub classes: IdMap<Class>,
    pub skills: IdMap<Skill>,
    pub effects: IdMap<Effect>,
    pub maps: IdMap<GameMap>,

    pub class_to_skills: HashMap<Id<Class>, Vec<Id<Skill>>>,
    pub skill_to_classes: HashMap<Id<Skill>, Vec<Id<Class>>>,
}
