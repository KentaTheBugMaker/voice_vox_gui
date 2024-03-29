//! definition of VoiceVox openapi path section
//!
//!

use crate::api_schema::{
    self, AccentPhrase, AccentPhrasesResponse, EngineManifestRaw, HttpValidationError,
    ParseKanaBadRequest, WordType,
};

use base64::{engine, Engine};
use once_cell::race::OnceBox;
use serde::Serialize;
use std::{collections::HashMap, convert::TryInto, io::ErrorKind};
use surf::{Error, StatusCode};

pub type CoreVersion = Option<String>;

///シングルトン surf::Client
static CLIENT: OnceBox<surf::Client> = once_cell::race::OnceBox::new();

///クライアントのシングルトンの作成/取得を行う.
fn client() -> &'static surf::Client {
    CLIENT.get_or_init(|| Box::new(surf::Client::new()))
}

/// # 音声合成用のクエリを作成する
///
/// クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味はSchemasを参照してください。
///
#[derive(Debug, Clone, Serialize)]
pub struct AudioQuery {
    pub text: String,
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl AudioQuery {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::AudioQuery, APIError> {
        let request = client()
            .post(format!("http://{server}/audio_query"))
            .query(&self)?
            .build();
        let mut res = client().send(request).await?;
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

///
/// # 音声合成用のクエリをプリセットを用いて作成する
/// クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味は`Schemas`を参照してください。
///
///
#[derive(Debug, Clone, Serialize)]
pub struct AudioQueryFromPreset {
    pub text: String,
    pub preset_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl AudioQueryFromPreset {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::AudioQuery, APIError> {
        let request = client()
            .post(format!("http://{server}/audio_query_from_preset"))
            .query(&self)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # テキストからアクセント句を得る
/// テキストからアクセント句を得ます。
///
/// is_kanaが`true`のとき、テキストは次のようなAquesTalkライクな記法に従う読み仮名として処理されます。デフォルトは`false`です。
///
/// * 全てのカナはカタカナで記述される
/// * アクセント句は`/`または`、`で区切る。`、`で区切った場合に限り無音区間が挿入される。
/// * カナの手前に`_`を入れるとそのカナは無声化される
/// * アクセント位置を`'`で指定する。全てのアクセント句にはアクセント位置を1つ指定する必要がある。
/// * アクセント句末に`？`(全角)を入れることにより疑問文の発音ができる。
///
#[derive(Debug, Clone, Serialize)]
pub struct AccentPhrases {
    pub text: String,
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_kana: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

#[derive(Debug)]
pub enum AccentPhrasesErrors {
    KanaParseError(ParseKanaBadRequest),
    ApiError(APIError),
}

impl From<surf::Error> for AccentPhrasesErrors {
    fn from(e: Error) -> Self {
        AccentPhrasesErrors::ApiError(e.into())
    }
}

impl From<surf::StatusCode> for AccentPhrasesErrors {
    fn from(e: surf::StatusCode) -> Self {
        AccentPhrasesErrors::ApiError(e.into())
    }
}

impl AccentPhrases {
    pub async fn call(self, server: &str) -> Result<AccentPhrasesResponse, AccentPhrasesErrors> {
        let request = client()
            .post(format!("http://{server}/accent_phrases"))
            .query(&self)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::BadRequest => Err(AccentPhrasesErrors::KanaParseError(
                res.body_json::<_>().await?,
            )),
            StatusCode::UnprocessableEntity => Err(AccentPhrasesErrors::ApiError(
                APIError::Validation(res.body_json::<_>().await?),
            )),
            x => Err(x.into()),
        }
    }
}

///アクセント句から音高を得る
#[derive(Debug, Clone, Serialize)]
pub struct MoraData {
    //in query
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    //in body
    #[serde(skip_serializing)]
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraData {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{server}/mora_data"))
            .query(&self)?
            .body_json(&self.accent_phrases)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # アクセント句から音素長を得る
#[derive(Debug, Clone, Serialize)]
pub struct MoraLength {
    // in query.
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body.
    #[serde(skip_serializing)]
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraLength {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{server}/mora_length"))
            .query(&self)?
            .body_json(&self.accent_phrases)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # アクセント句から音素長を得る
#[derive(Debug, Clone, Serialize)]
pub struct MoraPitch {
    // in query.
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body.
    #[serde(skip_serializing)]
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraPitch {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{server}/mora_pitch"))
            .query(&self)?
            .body_json(&self.accent_phrases)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<_>().await?),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # 音声合成する
#[derive(Debug, Clone, Serialize)]
pub struct Synthesis {
    // in query
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_interrogative_upspeak: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body body_json.
    #[serde(skip_serializing)]
    pub audio_query: crate::api_schema::AudioQuery,
}

impl Synthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{server}/synthesis"))
            .query(&self)?
            .body_json(&self.audio_query)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # 音声合成する（キャンセル可能）
#[derive(Debug, Clone, Serialize)]
pub struct CancellableSynthesis {
    // in query
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body body_json.
    #[serde(skip_serializing)]
    pub audio_query: crate::api_schema::AudioQuery,
}

impl CancellableSynthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{server}/cancellable_synthesis"))
            .query(&self)?
            .body_json(&self.audio_query)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # まとめて音声合成する
///
/// 複数のwavがzipでまとめられて返されます.
#[derive(Debug, Clone, Serialize)]
pub struct MultiSynthesis {
    // in query
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body body_json.
    #[serde(skip_serializing)]
    pub audio_query: Vec<crate::api_schema::AudioQuery>,
}

impl MultiSynthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{server}/multi_synthesis"))
            .query(&self)?
            .body_json(&self.audio_query)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # 2人の話者でモーフィングした音声を合成する
///
/// 指定された2人の話者で音声を合成、指定した割合でモーフィングした音声を得ます。 モーフィングの割合はmorph_rateで指定でき、0.0でベースの話者、1.0でターゲットの話者に近づきます。
#[derive(Debug, Clone, Serialize)]
pub struct SynthesisMorphing {
    // in query
    pub base_speaker: i32,
    pub target_speaker: i32,
    pub morph_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
    // in body body_json.
    #[serde(skip_serializing)]
    pub audio_query: crate::api_schema::AudioQuery,
}

impl SynthesisMorphing {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{server}/synthesis_morphing"))
            .query(&self)?
            .body_json(&self.audio_query)?
            .build();

        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

/// # base64エンコードされた複数のwavデータを一つに結合する
///
/// base64エンコードされたwavデータを一纏めにし、wavファイルで返します。
/// 返されるwavはbase64デコードを行います.
///
#[derive(Debug, Clone)]
pub struct ConnectWaves {
    pub waves: Vec<Vec<u8>>,
}

impl ConnectWaves {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let mut buffer = Vec::new();
        for wave in self.waves {
            buffer.push(engine::general_purpose::STANDARD.encode(wave));
        }
        let request = client()
            .post(format!("http://{server}/connect_waves"))
            .body_json(&buffer)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(engine::general_purpose::STANDARD
                .decode(res.body_string().await?.as_bytes())
                .unwrap_or_default()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Presets;

impl Presets {
    pub async fn call(self, server: &str) -> Result<Vec<crate::api_schema::Preset>, APIError> {
        let request = client().get(format!("http://{server}/presets")).build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<Vec<crate::api_schema::Preset>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version;

impl Version {
    pub async fn call(self, server: &str) -> Result<Option<String>, APIError> {
        let request = client().get(format!("http://{server}/version")).build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<Option<String>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoreVersions;

impl CoreVersions {
    pub async fn call(self, server: &str) -> Result<Vec<String>, APIError> {
        let request = client()
            .get(format!("http://{server}/core_versions"))
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res.body_json::<Vec<String>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Speakers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl Speakers {
    pub async fn call(self, server: &str) -> Result<Vec<crate::api_schema::Speaker>, APIError> {
        let request = client()
            .get(format!("http://{server}/speakers"))
            .query(&self)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => Ok(res.body_json::<Vec<crate::api_schema::Speaker>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpeakerInfo {
    pub speaker_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl SpeakerInfo {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::SpeakerInfo, APIError> {
        let req = client()
            .get(format!("http://{server}/speaker_info"))
            .query(&self)?
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => res
                .body_json::<crate::api_schema::SpeakerInfoRaw>()
                .await?
                .try_into()
                .map_err(|_| APIError::Io(std::io::Error::from(ErrorKind::InvalidData))),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SupportedDevices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl SupportedDevices {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::SupportedDevices, APIError> {
        let request = client()
            .get(format!("http://{server}/supported_devices"))
            .query(&self)?
            .build();
        let mut res = client().send(request).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => Ok(res
                .body_json::<crate::api_schema::SupportedDevices>()
                .await?),
            x => Err(x.into()),
        }
    }
}

/// Clone is implemented but Io and surf errors will converted to Unknown error.
///
#[derive(Debug)]
pub enum APIError {
    Validation(HttpValidationError),
    Io(std::io::Error),
    Surf(surf::Error),
    Unknown,
}

impl Clone for APIError {
    fn clone(&self) -> Self {
        match self {
            Self::Validation(arg0) => Self::Validation(arg0.clone()),
            Self::Io(_) => Self::Unknown,
            Self::Surf(_) => Self::Unknown,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl From<surf::Error> for APIError {
    fn from(e: Error) -> Self {
        APIError::Surf(e)
    }
}

impl From<std::io::Error> for APIError {
    fn from(val: std::io::Error) -> Self {
        APIError::Io(val)
    }
}

impl From<StatusCode> for APIError {
    fn from(_: StatusCode) -> Self {
        APIError::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct DownloadableLibraries;

impl DownloadableLibraries {
    pub async fn call(
        self,
        server: &str,
    ) -> Result<crate::api_schema::DownloadableLibraries, APIError> {
        let req = client()
            .get(format!("http://{server}/downloadble_libraries"))
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => res
                .body_json::<crate::api_schema::DownloadableLibrariesRaw>()
                .await?
                .try_into()
                .map_err(|_| APIError::Io(std::io::Error::from(ErrorKind::InvalidData))),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InitializeSpeaker {
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl InitializeSpeaker {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .post(format!("http://{server}/initialize_speaker"))
            .query(&self)?
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::NoContent => Ok(()),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IsInitializedSpeaker {
    pub speaker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: CoreVersion,
}

impl IsInitializedSpeaker {
    pub async fn call(self, server: &str) -> Result<bool, APIError> {
        let req = client()
            .get(format!("http://{server}/is_initialized_speaker"))
            .query(&self)?
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => Ok(res.body_json::<bool>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineManifest;

impl EngineManifest {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::EngineManifest, APIError> {
        let req = client()
            .get(format!("http://{server}/engine_manifest"))
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => res
                .body_json::<EngineManifestRaw>()
                .await?
                .try_into()
                .map_err(|_| APIError::Io(std::io::Error::from(ErrorKind::InvalidData))),
            x => Err(x.into()),
        }
    }
}

/// Get registered word list from user dictionary.
///
/// This result contains word UUID and definition.
///
#[derive(Debug, Clone)]
pub struct UserDict;

impl UserDict {
    pub async fn call(
        self,
        server: &str,
    ) -> Result<HashMap<String, crate::api_schema::UserDictWord>, APIError> {
        let req = client().get(format!("http://{server}/user_dict")).build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::Ok => Ok(res
                .body_json::<HashMap<String, crate::api_schema::UserDictWord>>()
                .await?),
            x => Err(x.into()),
        }
    }
}

/// Add word to user dictionary.
///
/// This result returns word UUID.
///
#[derive(Debug, Clone, Serialize)]
pub struct UserDictWord {
    ///言葉の表層形
    pub surface: String,
    ///言葉の発音(カタカナ)
    pub pronunciation: String,
    /// アクセント型(音が下がる場所)
    pub accent_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_type: Option<WordType>,
    ///単語の優先度
    ///     #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl UserDictWord {
    pub async fn call(self, server: &str) -> Result<String, APIError> {
        let req = client()
            .post(format!("http://{server}/user_dict_word"))
            .query(&self)?
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::Ok => Ok(res.body_string().await?),
            x => Err(x.into()),
        }
    }
}

/// rewrite word on user dictionary.
///
#[derive(Debug, Clone, Serialize)]
pub struct RewriteUserDictWord {
    /// word uuid
    pub uuid: String,
    ///言葉の表層形
    pub surface: String,
    ///言葉の発音(カタカナ)
    pub pronunciation: String,
    /// アクセント型(音が下がる場所)
    pub accent_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_type: Option<WordType>,
    ///単語の優先度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl RewriteUserDictWord {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .put(format!("http://{}/user_dict_word/{}", server, self.uuid))
            .query(&self)?
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::NoContent => Ok(()),
            x => Err(x.into()),
        }
    }
}

/// delete word from user dictionary.
///
///
#[derive(Debug, Clone)]
pub struct DeleteUserDictWord {
    /// word uuid
    pub uuid: String,
}

impl DeleteUserDictWord {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .delete(format!("http://{}/user_dict_word/{}", server, self.uuid))
            .build();
        let mut res = client().send(req).await.unwrap();
        match res.status() {
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            StatusCode::NoContent => Ok(()),
            x => Err(x.into()),
        }
    }
}

/// import user dictionary.
///
#[derive(Debug, Clone)]
pub struct ImportUserDict {
    pub over_ride: bool,
    pub dictionary: HashMap<String, api_schema::UserDictWord>,
}

impl ImportUserDict {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .post(format!("http://{server}/import_user_dict"))
            .body_json(&self.dictionary)?
            .build();
        let mut res = client().send(req).await?;
        match res.status() {
            StatusCode::NoContent => Ok(()),
            StatusCode::UnprocessableEntity => {
                Err(APIError::Validation(res.body_json::<_>().await?))
            }
            x => Err(x.into()),
        }
    }
}
