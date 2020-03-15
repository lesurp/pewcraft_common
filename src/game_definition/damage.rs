use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    flat_magical: Option<i32>,
    flat_physical: Option<i32>,
    flat_pure: Option<i32>,

    perc_magical: Option<f32>,
    perc_physical: Option<f32>,
    perc_pure: Option<f32>,
}
