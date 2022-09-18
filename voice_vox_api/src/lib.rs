//!
//! VoiceVox 0.11.4 api implementation.
//!

pub mod api;
pub mod api_schema;
#[cfg(test)]
mod test {
    use crate::api::{
        Api, AudioQuery, ConnectWaves, CoreVersions, MultiSynthesis, Presets, SpeakerInfo,
        Speakers, SupportedDevices, SynthesisMorphing, Version,
    };

    #[tokio::test]
    async fn call_multi_synthesis() {
        let aq0 = AudioQuery {
            text: "日本語".to_string(),
            speaker: 0,
            core_version: None,
        }
        .call("localhost:50021")
        .await
        .unwrap();
        let aq1 = AudioQuery {
            text: "音声合成".to_string(),
            speaker: 0,
            core_version: None,
        }
        .call("localhost:50021")
        .await
        .unwrap();
        MultiSynthesis {
            speaker: 0,
            core_version: None,
            audio_query: vec![aq0, aq1],
        }
        .call("localhost:50021")
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn call_synthesis_morphing() {
        let speakers: Vec<crate::api_schema::Speaker> = Speakers { core_version: None }
            .call("localhost:50021")
            .await
            .unwrap();
        let id_0 = speakers[0].styles[0].id;
        let id_1 = speakers[1].styles[0].id;

        let aq = AudioQuery {
            text: "音声合成".to_string(),
            speaker: id_0,
            core_version: None,
        }
        .call("localhost:50021")
        .await
        .unwrap();
        SynthesisMorphing {
            base_speaker: id_0,
            target_speaker: id_1,
            morph_rate: 0.5,
            core_version: None,
            audio_query: aq,
        }
        .call("localhost:50021")
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn call_connect_waves() {
        let waves = vec![];
        ConnectWaves { waves }
            .call("localhost:50021")
            .await
            .unwrap_or_default();
    }

    #[tokio::test]
    async fn call_presets() {
        let presets = Presets;
        presets.call("localhost:50021").await.unwrap();
    }

    #[tokio::test]
    async fn call_version() {
        let version = Version;
        version.call("localhost:50021").await.unwrap();
    }

    #[tokio::test]
    async fn call_core_versions() {
        let version = CoreVersions;
        version.call("localhost:50021").await.unwrap();
    }

    #[tokio::test]
    async fn call_speakers() {
        let speakers = Speakers { core_version: None };
        speakers.call("localhost:50021").await.unwrap();
    }

    #[tokio::test]
    async fn call_speaker_info() {
        let speakers = Speakers { core_version: None };
        let speakers = speakers.call("localhost:50021").await.unwrap();
        let info = SpeakerInfo {
            speaker_uuid: speakers[0].speaker_uuid.clone(),
            core_version: None,
        };
        info.call("localhost:50021").await.unwrap();
    }

    #[tokio::test]
    async fn call_supported_devices() {
        let supported_devices = SupportedDevices { core_version: None };
        supported_devices.call("localhost:50021").await.unwrap();
    }
}
