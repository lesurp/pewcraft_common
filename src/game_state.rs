
use crate::effect::{EffectKind, Range, RangeKind, Target};
use crate::map::{CellId, GameMapId};
use crate::skill::SkillId;
use crate::game_definition::GameDefinition;
use crate::id::Map;
use crate::character::{Character, CharacterId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

    pub fn player_to_play(&self) -> Option<CharacterId> {
        self.turn_order.last().cloned()
    }

    pub fn next_action(&mut self, g: &GameDefinition, ga: Action) -> Result<bool, ()> {
        match (ga, self.turn_state) {
            (Action::Skill(skill_id, cell_id), _) => self.execute_skill(g, skill_id, cell_id),
            (Action::Movement(cell_id), TurnState::MoveOrAction) => self.execute_move(g, cell_id),
            (_, _) => panic!("Wrong action :<<<"),
        }
    }

    fn player_at(&self, cell_id: CellId) -> Option<(&CharacterId, &Character)> {
        self.characters
            .iter()
            .find(|(_, character)| character.position == cell_id)
    }

    fn execute_skill(
        &mut self,
        g: &GameDefinition,
        skill_id: SkillId,
        cell_id: CellId,
    ) -> Result<bool, ()> {
        assert!(!self.turn_order.is_empty());
        let curr_char = self
            .characters
            .get(*self.turn_order.last().unwrap())
            .unwrap();
        let skill = g.skills.get(skill_id).unwrap();
        let target = self.player_at(cell_id);

        if !GameState::check_target(curr_char, &target, skill.range.target) {
            panic!("Wrong target!");
        }

        if !self.check_range(g, curr_char.position, cell_id, skill.range) {
            panic!("Wrong range...");
        }

        // TODO check LOS

        // FIXME: this is a borrow-checker workaround... but probably the only actual one..?
        let mut game_state_updates = HashSet::<(CharacterId, i32)>::new();
        // TODO: compute if hit?
        if true {
            if let Some((id, target)) = target {
                for effect in &skill.effects {
                    let effect = g.effects.get(*effect).unwrap();
                    match &effect.kind {
                        // TODO
                        EffectKind::Buff(_) => unimplemented!(),

                        // TODO: add somewhere if skills can attack other cells than just the
                        // target
                        EffectKind::DirectDamage(direct_damage) => {
                            let damage = direct_damage
                                .damage
                                .compute_damage(&g.classes, curr_char, target);
                            game_state_updates.insert((*id, damage));
                        }
                    }
                }
            }
        }

        // TODO stuff if dead
        for (id, damage) in game_state_updates {
            let character = self.characters.get_mut(id).unwrap();
            character.current_health -= damage;
        }

        self.turn_order.pop();
        Ok(self.turn_order.is_empty())
    }

    fn check_target(
        attacker: &Character,
        target_opt: &Option<(&CharacterId, &Character)>,
        target_kind: Target,
    ) -> bool {
        if let Some((_, target)) = target_opt {
            match (target_kind, target.team == attacker.team) {
                (Target::Anything, _) | (Target::Anyone, _) => true,
                (Target::Enemy, is_same_team) => !is_same_team,
                (Target::Ally, is_same_team) => is_same_team,
            }
        } else {
            Target::Anything == target_kind
        }
    }

    fn check_range(&self, g: &GameDefinition, start: CellId, end: CellId, range: Range) -> bool {
        let map = g.maps.get(self.map).unwrap();

        match range.kind {
            RangeKind::Star => {
                let distance = map.distance(start, end);
                distance <= range.max && distance >= range.min
            }
            RangeKind::Cross => {
                let (sx, sy) = map.id_to_xy_i32(start);
                let (ex, ey) = map.id_to_xy_i32(end);

                let dx = sx - ex;
                let dy = sy - ey;

                let max = range.max as i32;
                let min = range.min as i32;

                dx == 0 && dy.abs() <= max && dy.abs() >= min
                    || dy == 0 && dx.abs() <= max && dx.abs() >= min
            }
            RangeKind::Square => {
                let (sx, sy) = map.id_to_xy_i32(start);
                let (ex, ey) = map.id_to_xy_i32(end);

                let dx = sx - ex;
                let dy = sy - ey;

                let max = range.max as i32;
                let min = range.min as i32;

                dx.abs() <= max && dx.abs() >= min && dy >= min && dy <= max
            }
        }
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
