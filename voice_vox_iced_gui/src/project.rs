use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use voice_vox_api::api_schema;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AudioItem {
    pub text: String,
    pub styleId: i32,
    pub query: Option<api_schema::AudioQueryInProject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presetKey: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VoiceVoxProject {
    pub(crate) appVersion: String,
    pub audioKeys: Vec<String>,
    pub audioItems: HashMap<String, AudioItem>,
}
