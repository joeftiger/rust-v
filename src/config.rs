use crate::sensor::bounds::Bounds2;
use serde::{Deserialize, Serialize};
use ultraviolet::UVec2;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub filename: Option<String>,
    pub bounds: Option<Bounds2>,
    pub block_size: UVec2,
    pub passes: u32,
    pub threads: Option<u32>,
}
