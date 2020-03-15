use crate::id::Map;
pub use class::{Class, ClassId};
pub use damage::Damage;
pub use effect::{Buff, Effect};
pub use map::GameMap;
pub use serde::{Deserialize, Serialize};
pub use skill::{Skill, SkillId};
pub use std::collections::HashMap;

pub mod class;
pub mod damage;
pub mod effect;
pub mod map;
pub mod skill;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDefinition {
    pub classes: Map<Class>,
    pub skills: Map<Skill>,
    pub effects: Map<Effect>,
    pub maps: Map<GameMap>,

    pub class_to_skills: HashMap<ClassId, Vec<SkillId>>,
    pub skill_to_classes: HashMap<SkillId, Vec<ClassId>>,
}
