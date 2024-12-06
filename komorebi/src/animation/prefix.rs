use clap::ValueEnum;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use strum::EnumString;

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    ValueEnum,
    JsonSchema,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AnimationPrefix {
    Movement,
    Transparency,
}

pub fn new_animation_key(prefix: AnimationPrefix, key: String) -> String {
    format!("{}:{}", prefix, key)
}