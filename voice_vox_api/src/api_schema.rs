//! definition of VoiceVox openapi schema section.
#![allow(dead_code)]

use std::convert::{TryFrom, TryInto};

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

/// this is Used in all around.
///
///
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AudioQuery {
    pub accent_phrases: Vec<AccentPhrase>,
    pub speedScale: f32,
    pub pitchScale: f32,
    pub intonationScale: f32,
    pub volumeScale: f32,
    pub prePhonemeLength: f32,
    pub postPhonemeLength: f32,
    pub outputSamplingRate: i32,
    pub outputStereo: bool,
    pub kana: Option<String>,
}

/// this is used in AudioItem.
///
///
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AudioQueryInProject {
    pub accentPhrases: Vec<AccentPhraseInProject>,
    pub speedScale: f32,
    pub pitchScale: f32,
    pub intonationScale: f32,
    pub volumeScale: f32,
    pub prePhonemeLength: f32,
    pub postPhonemeLength: f32,
    pub outputSamplingRate: i32,
    pub outputStereo: bool,
    pub kana: Option<String>,
}

impl From<AudioQuery> for AudioQueryInProject {
    fn from(aq: AudioQuery) -> Self {
        Self {
            accentPhrases: aq
                .accent_phrases
                .iter()
                .map(|ap| ap.clone().into())
                .collect(),
            speedScale: aq.speedScale,
            pitchScale: aq.pitchScale,
            intonationScale: aq.intonationScale,
            volumeScale: aq.volumeScale,
            prePhonemeLength: aq.prePhonemeLength,
            postPhonemeLength: aq.postPhonemeLength,
            outputSamplingRate: aq.outputSamplingRate,
            outputStereo: aq.outputStereo,
            kana: aq.kana,
        }
    }
}

impl From<AudioQueryInProject> for AudioQuery {
    fn from(aq: AudioQueryInProject) -> Self {
        Self {
            accent_phrases: aq
                .accentPhrases
                .iter()
                .map(|ap| ap.clone().into())
                .collect(),
            speedScale: aq.speedScale,
            pitchScale: aq.pitchScale,
            intonationScale: aq.intonationScale,
            volumeScale: aq.volumeScale,
            prePhonemeLength: aq.prePhonemeLength,
            postPhonemeLength: aq.postPhonemeLength,
            outputSamplingRate: aq.outputSamplingRate,
            outputStereo: aq.outputStereo,
            kana: aq.kana,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccentPhrase {
    pub moras: Vec<Mora>,
    pub accent: i32,
    pub pause_mora: Option<Mora>,
    pub is_interrogative: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AccentPhraseInProject {
    pub moras: Vec<MoraInProject>,
    pub accent: i32,
    pub pause_mora: Option<MoraInProject>,
    pub isInterrogative: Option<bool>,
}

impl Serialize for AccentPhraseInProject {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("AccentPhrase", 4)?;
        s.serialize_field("moras", &self.moras)?;
        s.serialize_field("accent", &self.accent)?;
        if let Some(pause_mora) = &self.pause_mora {
            s.serialize_field("pause_mora", pause_mora)?;
        } else {
            s.skip_field("pause_mora")?;
        }
        s.serialize_field("isInterrogative", &self.isInterrogative.unwrap_or(false))?;
        s.end()
    }
}

impl From<AccentPhrase> for AccentPhraseInProject {
    fn from(ap: AccentPhrase) -> Self {
        Self {
            moras: ap.moras.iter().map(|mora| mora.clone().into()).collect(),
            accent: ap.accent,
            pause_mora: ap.pause_mora.map(|mora| mora.clone().into()),
            isInterrogative: ap.is_interrogative,
        }
    }
}

impl From<AccentPhraseInProject> for AccentPhrase {
    fn from(ap: AccentPhraseInProject) -> Self {
        Self {
            moras: ap.moras.iter().map(|mora| mora.clone().into()).collect(),
            accent: ap.accent,
            pause_mora: ap.pause_mora.map(|mora| mora.clone().into()),
            is_interrogative: ap.isInterrogative,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Mora {
    pub text: String,
    pub consonant: Option<String>,
    pub consonant_length: Option<f32>,
    pub vowel: String,
    pub vowel_length: f32,
    pub pitch: f32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MoraInProject {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub consonant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub consonantLength: Option<f32>,
    pub vowel: String,
    pub vowelLength: f32,
    pub pitch: f32,
}

impl From<Mora> for MoraInProject {
    fn from(mora: Mora) -> Self {
        Self {
            text: mora.text,
            consonant: mora.consonant,
            consonantLength: mora.consonant_length,
            vowel: mora.vowel,
            vowelLength: mora.vowel_length,
            pitch: mora.pitch,
        }
    }
}

impl From<MoraInProject> for Mora {
    fn from(mora: MoraInProject) -> Self {
        Self {
            text: mora.text.clone(),
            consonant: mora.consonant.clone(),
            consonant_length: mora.consonantLength,
            vowel: mora.vowel.clone(),
            vowel_length: mora.vowelLength,
            pitch: mora.pitch,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HttpValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ValidationError {
    ///Location
    pub loc: Vec<String>,
    ///Message
    pub msg: String,
    ///Error Type
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccentPhrasesResponse {
    pub accent_phrases: Vec<AccentPhrase>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParseKanaBadRequest {
    pub text: String,
    pub error_name: String,
    pub error_args: String,
}

pub enum ErrorName {
    UnknownText,
    AccentTop,
    AccentTwice,
    AccentNotFound,
    EmptyPhrase,
    InterrogationMarkNotAtEnd,
    InfiniteLoop,
}

#[allow(non_snake_case, unused_variables)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Preset {
    pub id: i32,
    pub name: String,
    pub speaker_uuid: String,
    pub style_id: i32,
    pub speedScale: f32,
    pub pitchScale: f32,
    pub intonationScale: f32,
    pub volumeScale: f32,
    pub prePhonemeLength: f32,
    pub postPhonemeLength: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Speaker {
    /// character name
    pub name: String,
    /// used to call SpeakerInfo.
    pub speaker_uuid: String,
    /// collection of emotion style.
    pub styles: Vec<SpeakerStyle>,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpeakerStyle {
    /// emotion style.
    pub name: String,
    /// style_id or speaker same as [StyleInfo.id]
    pub id: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct StyleInfoRaw {
    pub(crate) id: i32,
    /// base64
    pub(crate) icon: String,
    /// base64
    pub(crate) voice_samples: Vec<String>,
}

impl TryFrom<StyleInfoRaw> for StyleInfo {
    type Error = TryFromRawError;
    fn try_from(raw: StyleInfoRaw) -> Result<Self, <Self as TryFrom<StyleInfoRaw>>::Error> {
        Ok(Self {
            id: raw.id,
            icon: base64::decode(raw.icon).map_err(|_| TryFromRawError::Base64Decode)?,
            voice_samples: raw
                .voice_samples
                .iter()
                .filter_map(|b64| base64::decode(b64).ok())
                .collect(),
        })
    }
}

impl From<StyleInfo> for StyleInfoRaw {
    fn from(mut si: StyleInfo) -> Self {
        Self {
            id: si.id,
            icon: base64::encode(si.icon),
            voice_samples: si
                .voice_samples
                .drain(..)
                .map(|bytes| base64::encode(bytes))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct SpeakerInfo {
    /// markdown format.
    pub policy: String,
    /// png file.
    pub portrait: Vec<u8>,
    pub style_infos: Vec<StyleInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpeakerInfoRaw {
    pub(crate) policy: String,
    /// base64
    pub(crate) portrait: String,

    pub(crate) style_infos: Vec<StyleInfoRaw>,
}

pub enum TryFromRawError {
    Base64Decode,
}

impl TryFrom<SpeakerInfoRaw> for SpeakerInfo {
    type Error = TryFromRawError;
    fn try_from(mut raw: SpeakerInfoRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            policy: raw.policy,
            portrait: base64::decode(raw.portrait).map_err(|_| TryFromRawError::Base64Decode)?,
            style_infos: raw
                .style_infos
                .drain(..)
                .filter_map(|si_raw| si_raw.try_into().ok())
                .collect(),
        })
    }
}

impl From<SpeakerInfo> for SpeakerInfoRaw {
    fn from(mut si: SpeakerInfo) -> Self {
        Self {
            policy: si.policy,
            portrait: base64::encode(&si.portrait),
            style_infos: si.style_infos.drain(..).map(|si| si.into()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct StyleInfo {
    /// style_id or speaker. you can put into below API fields.
    /// * AudioQuery.speaker
    /// * AccentPhrases.speaker
    /// * MoraData.speaker
    /// * MoraPitch.speaker
    /// * MoraLength.speaker
    /// * Synthesis.speaker
    /// * CancellableSynthesis.speaker
    /// * MultiSynthesis.speaker
    /// * SynthesisMorphing.base_speaker
    /// * SynthesisMorphing.target_speaker
    pub id: i32,
    ///png file
    pub icon: Vec<u8>,
    ///wav file
    pub voice_samples: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SupportedDevices {
    /// always support
    pub cpu: bool,
    /// if enabled when Nvidia gpu + 3GiB VRam
    pub cuda: bool,
    /// if enabled when DirectML supported by engine.
    pub dml: Option<bool>,
}

#[derive(Debug)]
pub struct DownloadableLibraries {
    pub download_url: String,
    pub bytes: usize,
    pub speaker: Speaker,
    pub speaker_info: SpeakerInfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadableLibrariesRaw {
    download_url: String,
    bytes: usize,
    speaker: Speaker,
    speaker_info: SpeakerInfoRaw,
}

impl TryFrom<DownloadableLibrariesRaw> for DownloadableLibraries {
    type Error = TryFromRawError;

    fn try_from(value: DownloadableLibrariesRaw) -> Result<Self, Self::Error> {
        let speaker_info = value.speaker_info.try_into()?;
        Ok(Self {
            download_url: value.download_url,
            bytes: value.bytes,
            speaker: value.speaker,
            speaker_info,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineManifestRaw {
    manifest_version: String,
    name: String,
    uuid: String,
    url: String,
    icon: String,
    default_sampling_rate: i32,
    term_of_service: String,
    update_infos: Vec<UpdateInfo>,
    dependency_licenses: Vec<LicenseInfo>,
    downloadable_libraries_path: String,
    downloadable_libraries_url: String,
}

pub struct EngineManifest {
    pub manifest_version: String,
    pub name: String,
    pub uuid: String,
    pub url: String,
    pub icon: Vec<u8>,
    pub default_sampling_rate: i32,
    pub term_of_service: String,
    pub update_infos: Vec<UpdateInfo>,
    pub dependency_licenses: Vec<LicenseInfo>,
    pub downloadable_libraries_path: String,
    pub downloadable_libraries_url: String,
}

impl TryFrom<EngineManifestRaw> for EngineManifest {
    type Error = TryFromRawError;
    fn try_from(raw: EngineManifestRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            manifest_version: raw.manifest_version,
            name: raw.name,
            uuid: raw.uuid,
            url: raw.url,
            icon: base64::decode(raw.icon).map_err(|_| TryFromRawError::Base64Decode)?,
            default_sampling_rate: raw.default_sampling_rate,
            term_of_service: raw.term_of_service,
            update_infos: raw.update_infos,
            dependency_licenses: raw.dependency_licenses,
            downloadable_libraries_path: raw.downloadable_libraries_path,
            downloadable_libraries_url: raw.downloadable_libraries_url,
        })
    }
}

impl From<EngineManifest> for EngineManifestRaw {
    fn from(raw: EngineManifest) -> Self {
        Self {
            manifest_version: raw.manifest_version,
            name: raw.name,
            uuid: raw.uuid,
            url: raw.url,
            icon: base64::encode(raw.icon),
            default_sampling_rate: raw.default_sampling_rate,
            term_of_service: raw.term_of_service,
            update_infos: raw.update_infos,
            dependency_licenses: raw.dependency_licenses,
            downloadable_libraries_path: raw.downloadable_libraries_path,
            downloadable_libraries_url: raw.downloadable_libraries_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    version: String,
    descriptions: Vec<String>,
    contributers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseInfo {
    name: String,
    version: String,
    license: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDictWord {
    surface: String,
    priority: i32,
    context_id: Option<i32>,
    part_of_speech: String,
    part_of_speech_detail_1: String,
    part_of_speech_detail_2: String,
    part_of_speech_detail_3: String,
    inflectional_type: String,
    inflectional_form: String,
    stem: String,
    yomi: String,
    pronunciation: String,
    accent_type: i32,
    mora_count: Option<i32>,
    accent_associative_rule: String,
}

#[derive(Clone, Copy, Debug)]
pub enum WordType {
    ProperNoun,
    CommonNoun,
    Verb,
    Adjective,
    Suffix,
}

impl ToString for WordType {
    fn to_string(&self) -> String {
        match self {
            WordType::ProperNoun => "PROPER_NOUN",
            WordType::CommonNoun => "COMMON_NOUN",
            WordType::Verb => "VERB",
            WordType::Adjective => "ADJECTIVE",
            WordType::Suffix => "SUFFIX",
        }
        .to_owned()
    }
}
