use crate::game_definition::class::Attribute;
use crate::id::*;
use serde::{Deserialize, Serialize};

pub type BuffId = Id<Buff>;
pub type BuffMap = Map<Buff>;
pub type ClassMapBuilder = MapBuilder<Buff>;

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
    range: u32,

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
    // *all* skills apply buffs - direct damage are just buffs we apply directly
    Damage(ValueKind),
    // negative values mean healing i.e. HoT
    DoT(ValueKind),
    AttributeF(Attribute, ValueKind),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buff {
    // NOTE: we *require* ids during deserialisation, as those are required when deserialising
    // skills (as they applied the buffs)
    id: BuffId,
    name: String,
    range: Range,
    success_rate: f32,
    duration: i32,
    kind: BuffKind,
}
