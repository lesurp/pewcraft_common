use crate::class::*;
use crate::effect::Buff;
use crate::map::{CellId, Team, TeamId};
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type CharacterId = Id<Character>;
pub type CharacterMap = Map<Character>;

#[derive(Debug)]
pub struct CharacterMapBuilder {
    builder: MapBuilder<Character>,
    empty_starting_cells: Vec<(usize, HashSet<CellId>)>,
}

impl CharacterMapBuilder {
    pub fn new(teams: &[Team], team_size: usize) -> Self {
        CharacterMapBuilder {
            builder: MapBuilder::new(),
            empty_starting_cells: teams
                .iter()
                .map(|team| (team_size, team.1.clone()))
                .collect(),
        }
    }

    pub fn add(&mut self, c: Character) -> Result<(), ()> {
        let (spots_left, cells_left) = &mut self
            .empty_starting_cells
            .get_mut(c.team.raw())
            .expect("Wrong team id was assigned to the character");

        // Team is full
        if *spots_left == 0 {
            return Err(());
        }

        // Available cells left
        if !cells_left.remove(&c.position) {
            return Err(());
        }

        *spots_left -= 1;
        self.builder.add(c);
        Ok(())
    }

    pub fn build(self) -> Result<CharacterMap, Self> {
        for (spots_left, _) in &self.empty_starting_cells {
            if *spots_left > 0 {
                return Err(self);
            }
        }
        Ok(self.builder.build())
    }
}

// TODO: store the applier of the buff's stats here!
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuffInstance(Buff, ());

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub class: ClassId,

    pub current_health: i32,
    pub current_mana: i32,
    pub position: CellId,
    pub buffs: Vec<BuffInstance>,
    pub team: TeamId,
}

impl Character {
    pub fn new<S: Into<String>>(
        id: ClassId,
        position: CellId,
        class: &Class,
        name: S,
        team: TeamId,
    ) -> Character {
        Character {
            name: name.into(),
            class: id,
            current_health: class.health,
            current_mana: class.mana,
            position,
            buffs: Vec::new(),
            team,
        }
    }

    // TODO: look at current debuffs to return the "effective" stats
    pub fn effective_health(&self, class: &Class) -> i32 {
        class.health
    }

    pub fn effective_mana(&self, class: &Class) -> i32 {
        class.mana
    }

    pub fn effective_swiftness(&self, class: &Class) -> i32 {
        class.swiftness
    }

    // PHYSICAL STUFF
    pub fn effective_strength(&self, class: &Class) -> i32 {
        class.strength
    }

    pub fn effective_dexterity(&self, class: &Class) -> i32 {
        class.dexterity
    }

    pub fn effective_armor(&self, class: &Class) -> i32 {
        class.armor
    }

    // MAGICAL STUFF
    pub fn effective_intelligence(&self, class: &Class) -> i32 {
        class.intelligence
    }

    pub fn effective_concentration(&self, class: &Class) -> i32 {
        class.concentration
    }

    pub fn effective_willpower(&self, class: &Class) -> i32 {
        class.willpower
    }
}
