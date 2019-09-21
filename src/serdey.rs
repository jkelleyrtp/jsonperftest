use serde::{Deserialize, Serialize};

pub type ParticleState = serde_json::Value;

pub fn set_position(state: &mut ParticleState, x: f64, y: f64, z: f64) {
    *state.get_mut("position_x").unwrap() = json!(x);
    *state.get_mut("position_y").unwrap() = json!(y);
    *state.get_mut("position_z").unwrap() = json!(z);
}

pub fn set_velocity(state: &mut ParticleState, x: f64, y: f64, z: f64) {
    *state.get_mut("velocity_x").unwrap() = json!(x);
    *state.get_mut("velocity_y").unwrap() = json!(y);
    *state.get_mut("velocity_z").unwrap() = json!(z);
}


use serde_json::json;
pub fn new_electron() -> serde_json::Value {
    json!( {
        "position_x": 0.0,
        "position_y": 0.0,
        "position_z": 0.0,
        "velocity_x": 0.0,
        "velocity_y": 0.0,
        "velocity_z": 0.0,
        "mass": 1.0,
        "charge": -1.0
    })
}
