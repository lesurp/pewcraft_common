use crate::game_definition::class::Attribute;
use crate::id::*;
use serde::{Deserialize, Serialize};
use crate::game_definition::damage::Damage;

pub type EffectId = Id<Effect>;
pub type EffectMap = Map<Effect>;
pub type EffectMapBuilder = MapBuilder<Effect>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Anything, // can target empty cells e.g. for AoE
    Anyone,   // can target either allies or enemies
    Enemy,    // can only target enemies
    Ally,     // can only target allies
    // NOTE: a SelfOnly value is emulated using AllyOrSelf with range = 0
    AllyOrSelf, // can target allies or self
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RangeKind {
    // range is used as a L1 norm e.g. 1 means the four direct squares around the target are
    // affected as well
    Star,

    // straight line, in the four main directions
    Cross,

    // range is used for each x and y axis e.g. 1 means the 8 squared around the target are in the
    // range
    Square,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    // number of cells from initial one e.g. 0 means only on oneself
    min: u32,
    max: u32,

    // how should the area be covered
    kind: RangeKind,

    target: Target,

    // does this range needs line of sight (e.g. works through walls...)
    needs_los: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ValueKind {
    Perc(f32),
    Flat(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuffKind {
    // negative values mean healing i.e. HoT
    DoT(Damage),
    // TODO change that
    Attribute(Attribute, ValueKind),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buff {
    name: String,
    /// None means the damage will be applied to the given cell only (i.e. no "explosion" around
    /// the target)
    range: Option<Range>,
    success_rate: Option<f32>,
    duration: i32,
    kind: BuffKind,
    // TODO: add initial stats of attacker somewhere here
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectDamage {
    /// None means the damage will be applied to the given cell only (i.e. no "explosion" around
    /// the target)
    range: Option<Range>,
    damage: Damage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EffectKind {
    DirectDamage(DirectDamage),
    Buff(Buff),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    id: EffectId,
    kind: EffectKind,
}
