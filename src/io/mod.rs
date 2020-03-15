pub mod action;
pub mod character;

use crate::game_definition::map::{CellId, GameMapId};
use crate::game_definition::skill::SkillId;
use crate::game_definition::GameDefinition;
use crate::id::Map;
use character::{Character, CharacterId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TurnState {
    MoveOrAction,
    ActionOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Movement(CellId),
    Skill(SkillId, CellId),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    characters: Map<Character>,
    map: GameMapId,
    turn_order: Vec<CharacterId>,
    turn_state: TurnState,
}

impl GameState {
    pub fn new(g: &GameDefinition, characters: Map<Character>, map: GameMapId) -> GameState {
        let mut gs = GameState {
            characters,
            map,
            turn_order: Vec::new(),
            turn_state: TurnState::MoveOrAction,
        };
        gs.new_turn(g);
        gs
    }

    pub fn new_turn(&mut self, g: &GameDefinition) {
        assert!(self.turn_order.is_empty());

        let mut turn_order = self.characters.iter().collect::<Vec<_>>();
        turn_order.sort_by_key(|(_, character)| {
            let class = g.classes.get(character.class).unwrap();
            character.effective_swiftness(class)
        });
        self.turn_order = turn_order.iter().map(|(id, _)| **id).collect::<Vec<_>>();
    }

    pub fn next_action(&mut self, g: &GameDefinition, ga: Action) -> Result<bool, ()> {
        match (ga, self.turn_state) {
            (Action::Skill(skill_id, cell_id), _) => self.execute_skill(g, skill_id, cell_id),
            (Action::Movement(cell_id), TurnState::MoveOrAction) => self.execute_move(g, cell_id),
            (_, _) => panic!("Wrong action :<<<"),
        }
    }

    fn execute_skill(
        &mut self,
        g: &GameDefinition,
        skill_id: SkillId,
        cell_id: CellId,
    ) -> Result<bool, ()> {
        assert!(!self.turn_order.is_empty());
        let curr_char = self.characters.get(*self.turn_order.last().unwrap());
        let skill = g.skills.get(skill_id).unwrap();
        let target = g.maps.get(self.map).unwrap().id_to_xy(cell_id);

        // TODO check range...
        // TODO compute damage...

        self.turn_order.pop();
        Ok(self.turn_order.is_empty())
    }

    fn execute_move(&mut self, g: &GameDefinition, cell_id: CellId) -> Result<bool, ()> {
        assert!(!self.turn_order.is_empty());

        let curr_char = self
            .characters
            .get(*self.turn_order.last().unwrap())
            .unwrap();
        let class = g.classes.get(curr_char.class).unwrap();
        if g.maps.get(self.map).unwrap().can_move_to(
            curr_char.position,
            cell_id,
            curr_char.effective_swiftness(class),
        ) {
            self.turn_state = TurnState::ActionOnly;
            Ok(false)
        } else {
            panic!("Too far");
        }
    }
}
