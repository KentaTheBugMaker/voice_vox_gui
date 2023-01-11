//! definition of VoiceVox openapi path section
//!
//!

use crate::api_schema::{
    self, AccentPhrase, AccentPhrasesResponse, EngineManifestRaw, HttpValidationError,
    ParseKanaBadRequest, WordType,
};

use once_cell::race::OnceBox;
use reqwest::{Error, StatusCode};
use std::{collections::HashMap, convert::TryInto, io::ErrorKind};

pub type CoreVersion = Option<String>;

///シングルトン reqwest::Client
static CLIENT: OnceBox<reqwest::Client> = once_cell::race::OnceBox::new();

///クライアントのシングルトンの作成/取得を行う.
fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| Box::new(reqwest::Client::new()))
}

/// # 音声合成用のクエリを作成する
///
/// クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味はSchemasを参照してください。
///
#[derive(Debug, Clone)]
pub struct AudioQuery {
    pub text: String,
    pub speaker: i32,
    pub core_version: CoreVersion,
}

impl AudioQuery {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::AudioQuery, APIError> {
        let request = client()
            .post(format!("http://{}/audio_query", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&&self.core_version)
            .query(&[("text", self.text)])
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

///
/// # 音声合成用のクエリをプリセットを用いて作成する
/// クエリの初期値を得ます。ここで得られたクエリはそのまま音声合成に利用できます。各値の意味は`Schemas`を参照してください。
///
///
#[derive(Debug, Clone)]
pub struct AudioQueryFromPreset {
    pub text: String,
    pub preset_id: i32,
    pub core_version: CoreVersion,
}

impl AudioQueryFromPreset {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::AudioQuery, APIError> {
        let request = client()
            .post(format!("http://{}/audio_query_from_preset", server))
            .query(&[("preset_id", self.preset_id)])
            .add_core_version(&&self.core_version)
            .query(&[("text", self.text)])
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
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
#[derive(Debug, Clone)]
pub struct AccentPhrases {
    pub text: String,
    pub speaker: i32,
    pub is_kana: Option<bool>,
    pub core_version: CoreVersion,
}

#[derive(Debug)]
pub enum AccentPhrasesErrors {
    KanaParseError(ParseKanaBadRequest),
    ApiError(APIError),
}

impl From<reqwest::Error> for AccentPhrasesErrors {
    fn from(e: Error) -> Self {
        AccentPhrasesErrors::ApiError(e.into())
    }
}

impl From<reqwest::StatusCode> for AccentPhrasesErrors {
    fn from(e: reqwest::StatusCode) -> Self {
        AccentPhrasesErrors::ApiError(e.into())
    }
}

impl AccentPhrases {
    pub async fn call(self, server: &str) -> Result<AccentPhrasesResponse, AccentPhrasesErrors> {
        let request = client()
            .post(format!("http://{}/accent_phrases", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .query(&[("is_kana", self.is_kana.unwrap_or(false))])
            .query(&[("text", self.text)])
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::BAD_REQUEST => {
                Err(AccentPhrasesErrors::KanaParseError(res.json::<_>().await?))
            }
            StatusCode::UNPROCESSABLE_ENTITY => Err(AccentPhrasesErrors::ApiError(
                APIError::Validation(res.json::<_>().await?),
            )),
            x => Err(x.into()),
        }
    }
}

///アクセント句から音高を得る
#[derive(Debug, Clone)]
pub struct MoraData {
    //in query
    pub speaker: i32,
    pub core_version: CoreVersion,
    //in body
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraData {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{}/mora_data", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .json(&self.accent_phrases)
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # アクセント句から音素長を得る
#[derive(Debug, Clone)]
pub struct MoraLength {
    // in query.
    pub speaker: i32,
    pub core_version: CoreVersion,
    // in body.
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraLength {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{}/mora_length", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .json(&self.accent_phrases)
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # アクセント句から音素長を得る
#[derive(Debug, Clone)]
pub struct MoraPitch {
    // in query.
    pub speaker: i32,
    pub core_version: CoreVersion,
    // in body.
    pub accent_phrases: Vec<AccentPhrase>,
}

impl MoraPitch {
    pub async fn call(self, server: &str) -> Result<Vec<AccentPhrase>, APIError> {
        let request = client()
            .post(format!("http://{}/mora_pitch", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .json(&self.accent_phrases)
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<_>().await?),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # 音声合成する
#[derive(Debug, Clone)]
pub struct Synthesis {
    // in query
    pub speaker: i32,
    pub enable_interrogative_upspeak: Option<bool>,
    pub core_version: CoreVersion,
    // in body json.
    pub audio_query: crate::api_schema::AudioQuery,
}

impl Synthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{}/synthesis", server))
            .query(&[("speaker", self.speaker)])
            .query(&[(
                "enable_interrogative_upspeak",
                self.enable_interrogative_upspeak.unwrap_or(true),
            )])
            .add_core_version(&self.core_version)
            .json(&self.audio_query)
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # 音声合成する（キャンセル可能）
#[derive(Debug, Clone)]
pub struct CancellableSynthesis {
    // in query
    pub speaker: i32,
    pub core_version: CoreVersion,
    // in body json.
    pub audio_query: crate::api_schema::AudioQuery,
}

impl CancellableSynthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{}/cancellable_synthesis", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .json(&self.audio_query)
            .build()?;
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # まとめて音声合成する
///
/// 複数のwavがzipでまとめられて返されます.
#[derive(Debug, Clone)]
pub struct MultiSynthesis {
    // in query
    pub speaker: i32,
    pub core_version: CoreVersion,
    // in body json.
    pub audio_query: Vec<crate::api_schema::AudioQuery>,
}

impl MultiSynthesis {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{}/multi_synthesis", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .json(&self.audio_query)
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

/// # 2人の話者でモーフィングした音声を合成する
///
/// 指定された2人の話者で音声を合成、指定した割合でモーフィングした音声を得ます。 モーフィングの割合はmorph_rateで指定でき、0.0でベースの話者、1.0でターゲットの話者に近づきます。
#[derive(Debug, Clone)]
pub struct SynthesisMorphing {
    // in query
    pub base_speaker: i32,
    pub target_speaker: i32,
    pub morph_rate: f64,
    pub core_version: CoreVersion,
    // in body json.
    pub audio_query: crate::api_schema::AudioQuery,
}

impl SynthesisMorphing {
    pub async fn call(self, server: &str) -> Result<Vec<u8>, APIError> {
        let request = client()
            .post(format!("http://{}/synthesis_morphing", server))
            .query(&[
                ("base_speaker", self.base_speaker),
                ("target_speaker", self.target_speaker),
            ])
            .query(&[("morph_rate", self.morph_rate)])
            .add_core_version(&self.core_version)
            .json(&self.audio_query)
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.bytes().await.unwrap_or_default().to_vec()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
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
            buffer.push(base64::encode(wave));
        }
        let request = client()
            .post(format!("http://{}/connect_waves", server))
            .json(&buffer)
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(base64::decode(res.text().await?).unwrap_or_default()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Presets;

impl Presets {
    pub async fn call(self, server: &str) -> Result<Vec<crate::api_schema::Preset>, APIError> {
        let request = client()
            .get(format!("http://{}/presets", server))
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<crate::api_schema::Preset>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version;

impl Version {
    pub async fn call(self, server: &str) -> Result<Option<String>, APIError> {
        let request = client()
            .get(format!("http://{}/version", server))
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<Option<String>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoreVersions;

impl CoreVersions {
    pub async fn call(self, server: &str) -> Result<Vec<String>, APIError> {
        let request = client()
            .get(format!("http://{}/core_versions", server))
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<String>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Speakers {
    pub core_version: CoreVersion,
}

impl Speakers {
    pub async fn call(self, server: &str) -> Result<Vec<crate::api_schema::Speaker>, APIError> {
        let request = client()
            .get(format!("http://{}/speakers", server))
            .add_core_version(&self.core_version)
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => Ok(res.json::<Vec<crate::api_schema::Speaker>>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpeakerInfo {
    pub speaker_uuid: String,
    pub core_version: CoreVersion,
}

impl SpeakerInfo {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::SpeakerInfo, APIError> {
        let req = client()
            .get(format!("http://{}/speaker_info", server))
            .query(&[("speaker_uuid", self.speaker_uuid)])
            .add_core_version(&self.core_version)
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => res
                .json::<crate::api_schema::SpeakerInfoRaw>()
                .await?
                .try_into()
                .map_err(|_| APIError::Io(std::io::Error::from(ErrorKind::InvalidData))),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SupportedDevices {
    pub core_version: CoreVersion,
}

impl SupportedDevices {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::SupportedDevices, APIError> {
        let request = client()
            .get(format!("http://{}/supported_devices", server))
            .add_core_version(&self.core_version)
            .build()
            .unwrap();
        let res = client().execute(request).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => Ok(res.json::<crate::api_schema::SupportedDevices>().await?),
            x => Err(x.into()),
        }
    }
}

pub trait AddCoreVersion {
    fn add_core_version(self, core_version: &CoreVersion) -> Self;
}

impl AddCoreVersion for reqwest::RequestBuilder {
    fn add_core_version(self, core_version: &CoreVersion) -> Self {
        if let Some(cv) = &core_version {
            self.query(&[("core_version", cv)])
        } else {
            self
        }
    }
}

/// Clone is implemented but Io and Reqwest errors will converted to Unknown error.
///
#[derive(Debug)]
pub enum APIError {
    Validation(HttpValidationError),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Unknown,
}

impl Clone for APIError {
    fn clone(&self) -> Self {
        match self {
            Self::Validation(arg0) => Self::Validation(arg0.clone()),
            Self::Io(_) => Self::Unknown,
            Self::Reqwest(_) => Self::Unknown,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl From<reqwest::Error> for APIError {
    fn from(e: Error) -> Self {
        APIError::Reqwest(e)
    }
}

impl Into<APIError> for std::io::Error {
    fn into(self) -> APIError {
        APIError::Io(self)
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
            .get(format!("http://{}/downloadble_libraries", server))
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => res
                .json::<crate::api_schema::DownloadableLibrariesRaw>()
                .await?
                .try_into()
                .map_err(|_| APIError::Io(std::io::Error::from(ErrorKind::InvalidData))),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InitializeSpeaker {
    pub speaker: i32,
    pub core_version: CoreVersion,
}

impl InitializeSpeaker {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .post(format!("http://{}/initialize_speaker", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::NO_CONTENT => Ok(()),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IsInitializedSpeaker {
    pub speaker: i32,
    pub core_version: CoreVersion,
}

impl IsInitializedSpeaker {
    pub async fn call(self, server: &str) -> Result<bool, APIError> {
        let req = client()
            .get(format!("http://{}/is_initialized_speaker", server))
            .query(&[("speaker", self.speaker)])
            .add_core_version(&self.core_version)
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => Ok(res.json::<bool>().await?),
            x => Err(x.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineManifest;

impl EngineManifest {
    pub async fn call(self, server: &str) -> Result<crate::api_schema::EngineManifest, APIError> {
        let req = client()
            .get(format!("http://{}/engine_manifest", server))
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => res
                .json::<EngineManifestRaw>()
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
        let req = client()
            .get(format!("http://{}/user_dict", server))
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res
                .json::<HashMap<String, crate::api_schema::UserDictWord>>()
                .await?),
            x => Err(x.into()),
        }
    }
}

/// Add word to user dictionary.
///
/// This result returns word UUID.
///
#[derive(Debug, Clone)]
pub struct UserDictWord {
    ///言葉の表層形
    pub surface: String,
    ///言葉の発音(カタカナ)
    pub pronunciation: String,
    /// アクセント型(音が下がる場所)
    pub accent_type: i32,
    pub word_type: Option<WordType>,
    ///単語の優先度
    pub priority: Option<i32>,
}

impl UserDictWord {
    pub async fn call(self, server: &str) -> Result<String, APIError> {
        let req = client()
            .post(format!("http://{}/user_dict_word", server))
            .query(&[
                ("surface", self.surface),
                ("pronunciation", self.pronunciation),
            ])
            .query(&[("accent_type", self.accent_type)]);
        let req = if let Some(priority) = self.priority {
            req.query(&[("priority", priority)])
        } else {
            req
        };
        let req = if let Some(word_type) = self.word_type {
            req.query(&[("word_type", word_type.to_string())])
        } else {
            req
        }
        .build()
        .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::OK => Ok(res.text().await?),
            x => Err(x.into()),
        }
    }
}

/// rewrite word on user dictionary.
///
#[derive(Debug, Clone)]
pub struct RewriteUserDictWord {
    /// word uuid
    pub uuid: String,
    ///言葉の表層形
    pub surface: String,
    ///言葉の発音(カタカナ)
    pub pronunciation: String,
    /// アクセント型(音が下がる場所)
    pub accent_type: i32,
    pub word_type: Option<WordType>,
    ///単語の優先度
    pub priority: Option<i32>,
}

impl RewriteUserDictWord {
    pub async fn call(self, server: &str) -> Result<(), APIError> {
        let req = client()
            .put(format!("http://{}/user_dict_word/{}", server, self.uuid))
            .query(&[
                ("surface", self.surface),
                ("pronunciation", self.pronunciation),
            ])
            .query(&[("accent_type", self.accent_type)]);
        let req = if let Some(priority) = self.priority {
            req.query(&[("priority", priority)])
        } else {
            req
        };
        let req = if let Some(word_type) = self.word_type {
            req.query(&[("word_type", word_type.to_string())])
        } else {
            req
        }
        .build()
        .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::NO_CONTENT => Ok(()),
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
            .build()
            .unwrap();
        let res = client().execute(req).await.unwrap();
        match res.status() {
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            StatusCode::NO_CONTENT => Ok(()),
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
            .post(format!("http://{}/import_user_dict", server))
            .json(&self.dictionary)
            .build()
            .unwrap();
        let res = client().execute(req).await?;
        match res.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::UNPROCESSABLE_ENTITY => Err(APIError::Validation(res.json::<_>().await?)),
            x => Err(x.into()),
        }
    }
}
