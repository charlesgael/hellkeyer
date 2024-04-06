use std::collections::HashMap;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::stratagem::Stratagem;

#[derive(Debug, Default, Deserialize)]
pub struct AppState {
    pub config: Config,
    pub mappings: RwLock<Mapping>,
    pub stratagems: Vec<Vec<Stratagem>>,
    pub stratagems_map: HashMap<String, Stratagem>
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Mapping {
    pub f5: Option<String>,
    pub f6: Option<String>,
    pub f7: Option<String>,
    pub f8: Option<String>,
    pub f9: Option<String>,
    pub f10: Option<String>,
}