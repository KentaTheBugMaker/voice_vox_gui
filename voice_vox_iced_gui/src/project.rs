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
impl Default for VoiceVoxProject {
    fn default() -> Self {
        let audio_item = AudioItem {
            text: String::new(),
            styleId: 0,
            query: Some(voice_vox_api::api_schema::AudioQuery::default().into()),
            presetKey: None,
        };

        let uuid = uuid::Uuid::new_v4().to_string();
        let keys = vec![uuid.clone()];
        let mut items = HashMap::new();
        items.insert(uuid, audio_item);

        VoiceVoxProject {
            appVersion: "0.13.3".to_owned(),
            audioKeys: keys,
            audioItems: items,
        }
    }
}
