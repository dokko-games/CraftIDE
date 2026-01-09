use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::models::version::Version;
use crate::models::generator::Generator;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub mod_id: String,
    pub author: String,
    pub path: PathBuf,
    pub target_version: Version,
    pub description: String,
    pub generators: Vec<Generator>
}