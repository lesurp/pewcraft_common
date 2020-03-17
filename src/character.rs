use crate::class::Class;
use crate::effect::Buff;
use crate::id_map::{Id, IdMap, IdMapBuilder};
use crate::map::Cell;
use crate::map::Team;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug)]
pub struct CharacterMapBuilder {
    builder: IdMapBuilder<Character>,
    empty_starting_cells: Vec<(usize, HashSet<Id<Cell>>)>,
}

impl CharacterMapBuilder {
    pub fn new(teams: &[Team], team_size: usize) -> Self {
        CharacterMapBuilder {
            builder: IdMapBuilder::new(),
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

    pub fn build(self) -> Result<IdMap<Character>, Self> {
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
    pub class: Id<Class>,

    pub current_health: i32,
    pub current_mana: i32,
    pub position: Id<Cell>,
    pub buffs: Vec<BuffInstance>,
    pub team: Id<Team>,
}

impl Character {
    pub fn new<S: Into<String>>(
        id: Id<Class>,
        position: Id<Cell>,
        class: &Class,
        name: S,
        team: Id<Team>,
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
