mod main_page;
mod project;
mod toolbar;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use iced::widget::pane_grid::{self, State as PaneGridState};
use iced::{
    widget::{self, column, Button, Column, PickList, Row, Text},
    Application, Command, Element, Settings, Theme,
};
use main_page::InTabPane;
use project::AudioItem;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use toolbar::{build_configure_ui, ConfigureMessage, ToolBarConfig, ToolBarKind};
use voice_vox_api::api::{Api, SpeakerInfo};
fn main() -> iced::Result {
    VoiceVox::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansCJKjp-Regular.otf")),
        ..Default::default()
    })
}
static SERVER: once_cell::sync::OnceCell<&str> = once_cell::sync::OnceCell::new();

#[derive(Debug, Clone)]
pub(crate) enum Message {
    FileMenuOpen(FileMenu),
    EngineMenuOpen(EngineMenu),
    SettingsMenuOpen(SettingsMenu),
    HelpMenuOpen,
    ToolBar(ToolBarKind),
    Loaded(Result<VoiceVoxState, LoadError>),
    Saved(Result<(), SaveError>),
    ToolBarConfig(ConfigureMessage),
    IntabPaneResize(iced::widget::pane_grid::ResizeEvent),
    TabSelect(usize),
    TabClose(usize),
    EditText(String, String),
    APICall(APICall),
    APIResult(APIResult),
}
#[derive(Debug, Clone)]
pub(crate) enum APIResult {
    Speakers(<voice_vox_api::api::Speakers as voice_vox_api::api::Api>::Response),
    SpeakerInfo(
        voice_vox_api::api_schema::Speaker,
        <voice_vox_api::api::SpeakerInfo as voice_vox_api::api::Api>::Response,
    ),
}

#[derive(Debug, Clone)]
pub(crate) enum APICall {
    Speakers(voice_vox_api::api::Speakers),
    SpeakerInfo(
        voice_vox_api::api_schema::Speaker,
        voice_vox_api::api::SpeakerInfo,
    ),
}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct VoiceVoxState {
    tool_bar_config: ToolBarConfig,
    viewing_tab: Option<usize>,
    tabs: Vec<TabContext>,
}
enum VoiceVox {
    Loading,
    Loaded(State),
}
impl Application for VoiceVox {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;
    fn new(_flags: Self::Flags) -> (VoiceVox, Command<Message>) {
        (
            VoiceVox::Loading,
            Command::perform(VoiceVoxState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        match self {
            VoiceVox::Loading => "Loading".to_owned(),
            VoiceVox::Loaded(state) => {
                format!(
                    "Voiced - {}",
                    state.persistence.get_tab_file_name().unwrap_or_default()
                )
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            Self::Loading => {
                SERVER.set("localhost:50021").unwrap();
                // initialize pane
                // character and other split.
                let configure = pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Horizontal,
                    ratio: 0.5,
                    a: Box::new(pane_grid::Configuration::Split {
                        axis: pane_grid::Axis::Vertical,
                        ratio: 0.5,
                        a: Box::new(pane_grid::Configuration::Pane(InTabPane::CharacterPane)),
                        b: Box::new(pane_grid::Configuration::Split {
                            axis: pane_grid::Axis::Vertical,
                            ratio: 0.5,
                            a: Box::new(pane_grid::Configuration::Pane(InTabPane::TextPane)),
                            b: Box::new(pane_grid::Configuration::Pane(InTabPane::ParameterPane)),
                        }),
                    }),
                    b: Box::new(pane_grid::Configuration::Pane(InTabPane::BottomPane)),
                };
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Self::Loaded(State {
                            dirty: false,
                            saving: false,
                            persistence: state,
                            opening_page: Page::ToolBarConfig,
                            configure_ui_selected_tool: ToolBarKind::default(),
                            toolbar_ui_temp_config: ToolBarConfig::default(),

                            tab_state: PaneGridState::with_configuration(configure),
                            portrait_and_names: HashMap::new(),
                            style_id_uuid_table: BTreeMap::new(),
                            icons: BTreeMap::new(),
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Self::Loaded(State {
                            dirty: false,
                            saving: false,
                            persistence: VoiceVoxState::default(),
                            opening_page: Page::ToolBarConfig,
                            configure_ui_selected_tool: ToolBarKind::default(),
                            toolbar_ui_temp_config: ToolBarConfig::default(),

                            tab_state: PaneGridState::with_configuration(configure),
                            portrait_and_names: HashMap::new(),
                            style_id_uuid_table: BTreeMap::new(),
                            icons: BTreeMap::new(),
                        });
                    }
                    _ => {}
                }
                // collect informations from engine.
                Command::perform(
                    voice_vox_api::api::Speakers { core_version: None }.call("localhost:50021"),
                    |res| Message::APIResult(APIResult::Speakers(res)),
                )
            }
            Self::Loaded(state) => {
                let mut saved = false;
                let mut cmd_buff = vec![];

                match message {
                    Message::FileMenuOpen(file_menu) => match file_menu {
                        FileMenu::ExportAll => todo!(),
                        FileMenu::ExportSelected => todo!(),
                        FileMenu::ExportConnected => todo!(),
                        FileMenu::ExportTextConnected => todo!(),
                        FileMenu::ImportText => todo!(),
                        FileMenu::NewProject => state.persistence.tabs.push(TabContext::default()),
                        FileMenu::SaveProject => todo!(),
                        FileMenu::SaveProjectAs => todo!(),
                        FileMenu::LoadProject => todo!(),
                    },
                    Message::EngineMenuOpen(_) => {}
                    Message::SettingsMenuOpen(settings_menu) => match settings_menu {
                        SettingsMenu::KeyConfig => todo!(),
                        SettingsMenu::ToolbarConfig => {
                            state.opening_page = Page::ToolBarConfig;
                            state.toolbar_ui_temp_config =
                                state.persistence.tool_bar_config.clone();
                        }
                        SettingsMenu::ReorderCharacter => todo!(),
                        SettingsMenu::DefaultStyle => todo!(),
                        SettingsMenu::Dictionary => todo!(),
                        SettingsMenu::Option => todo!(),
                    },
                    Message::HelpMenuOpen => {}
                    Message::ToolBar(_) => {}
                    Message::Loaded(_) => {}
                    Message::Saved(result) => {
                        if let Ok(()) = result {
                            saved = true;
                            state.saving = false;
                        }
                    }
                    Message::ToolBarConfig(message) => match message {
                        ConfigureMessage::Toggle(kind, false) => {
                            state.toolbar_ui_temp_config.remove(kind)
                        }
                        ConfigureMessage::Toggle(kind, true) => {
                            state.toolbar_ui_temp_config.insert(kind)
                        }
                        ConfigureMessage::RestoreDefault => {
                            state.toolbar_ui_temp_config = Default::default();
                        }
                        ConfigureMessage::Save => {
                            state.persistence.tool_bar_config =
                                state.toolbar_ui_temp_config.clone();
                            state.dirty = true;
                        }
                        ConfigureMessage::Exit => {
                            state.opening_page = Page::Main;
                        }
                        ConfigureMessage::MoveLeft => {
                            state
                                .toolbar_ui_temp_config
                                .move_left(state.configure_ui_selected_tool);
                        }
                        ConfigureMessage::MoveRight => {
                            state
                                .toolbar_ui_temp_config
                                .move_right(state.configure_ui_selected_tool);
                        }
                        ConfigureMessage::Remove => state
                            .toolbar_ui_temp_config
                            .remove(state.configure_ui_selected_tool),
                        ConfigureMessage::Select(kind) => {
                            state.configure_ui_selected_tool = kind;
                        }
                    },

                    Message::IntabPaneResize(event) => {
                        state.tab_state.resize(&event.split, event.ratio)
                    }
                    Message::TabSelect(tab_id) => {
                        state.persistence.viewing_tab = Some(tab_id);
                    }
                    Message::TabClose(tab_id) => {
                        let tab_ctx = state.persistence.tabs.remove(tab_id);

                        /*
                        let save = Command::perform(
                            tokio::fs::write(
                                tab_ctx.file_name,
                                serde_json::ser::to_vec_pretty(&tab_ctx.project).unwrap(),
                            ),
                            Message::Saved,
                        );
                        */
                    }
                    Message::EditText(key, text) => {
                        if let Some(tab_ctx) = state
                            .persistence
                            .tabs
                            .get_mut(state.persistence.viewing_tab.unwrap_or(0))
                        {
                            if let Some(audio_item) = tab_ctx.project.audioItems.get_mut(&key) {
                                audio_item.text = text;
                            }
                        }
                    }
                    Message::APICall(request) => match request {
                        APICall::Speakers(_) => {}
                        APICall::SpeakerInfo(speaker, si) => {
                            let si = Box::new(si);
                            let si = Box::leak(si);

                            return Command::perform(si.call(SERVER.get().unwrap()), |result| {
                                Message::APIResult(APIResult::SpeakerInfo(speaker, result))
                            });
                        }
                    },
                    Message::APIResult(result) => match result {
                        APIResult::Speakers(speakers) => {
                            println!("{:?}", speakers);

                            if let Ok(speakers) = speakers {
                                for speaker in speakers.clone() {
                                    cmd_buff.push(Command::perform(async {}, |_| {
                                        Message::APICall(APICall::SpeakerInfo(
                                            speaker.clone(),
                                            SpeakerInfo {
                                                speaker_uuid: speaker.speaker_uuid,
                                                core_version: None,
                                            },
                                        ))
                                    }));
                                }
                            }
                        }
                        APIResult::SpeakerInfo(speaker, info) => {
                            if let Ok(info) = info {
                                state.portrait_and_names.insert(
                                    speaker.speaker_uuid.clone(),
                                    (
                                        widget::image::Handle::from_memory(info.portrait),
                                        speaker.name,
                                    ),
                                );
                                for style in speaker.styles {
                                    style.name;
                                }
                                for style in info.style_infos {
                                    let id = style.id;

                                    state
                                        .style_id_uuid_table
                                        .insert(id, speaker.speaker_uuid.clone());
                                    state.icons.insert(
                                        style.id,
                                        widget::image::Handle::from_memory(style.icon),
                                    );
                                }
                            }
                        }
                    },
                }

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(state.persistence.clone().save(), Message::Saved)
                } else {
                    Command::none()
                };
                cmd_buff.push(save);
                Command::batch(cmd_buff)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if let Self::Loaded(state) = self {
            let vbox = Column::new();
            // menubar
            let top_bar = widget::Row::new()
                .push(
                    PickList::new(
                        if state.opening_page == Page::Main {
                            enumerate_file_menu()
                        } else {
                            vec![]
                        },
                        None,
                        Message::FileMenuOpen,
                    )
                    .placeholder("ファイル".to_owned()),
                )
                .push(
                    PickList::new(
                        if state.opening_page == Page::Main {
                            enumerate_engine_menu()
                        } else {
                            vec![]
                        },
                        None,
                        Message::EngineMenuOpen,
                    )
                    .placeholder("エンジン".to_owned()),
                )
                .push(
                    PickList::new(
                        if state.opening_page == Page::Main {
                            enumerate_settings_menu()
                        } else {
                            vec![]
                        },
                        None,
                        Message::SettingsMenuOpen,
                    )
                    .placeholder("設定".to_owned()),
                )
                .push({
                    let button = Button::new(Text::new("Help"));
                    if state.opening_page == Page::Main {
                        button.on_press(Message::HelpMenuOpen)
                    } else {
                        button
                    }
                });

            let top_bar = if let Self::Loaded(state) = self {
                top_bar.push(Text::new(
                    state.persistence.get_tab_file_name().unwrap_or_default(),
                ))
            } else {
                top_bar
            };

            let page = match state.opening_page {
                Page::Main => {
                    // tab bar

                    main_page::build_ui(
                        &state.persistence.tool_bar_config,
                        &state.tab_state,
                        state.persistence.viewing_tab.unwrap_or(0),
                        &state.persistence.tabs,
                        &state.portrait_and_names,
                        &state.style_id_uuid_table,
                        &state.icons,
                    )
                }
                Page::ToolBarConfig => build_configure_ui(
                    &state.persistence.tool_bar_config,
                    &state.toolbar_ui_temp_config,
                    &state.configure_ui_selected_tool,
                ),
                Page::Help => column(vec![]),
            };

            vbox.push(top_bar).push(page).into()
        } else {
            Row::new().into()
        }
    }
}
struct State {
    dirty: bool,
    saving: bool,
    opening_page: Page,
    configure_ui_selected_tool: ToolBarKind,
    toolbar_ui_temp_config: ToolBarConfig,
    persistence: VoiceVoxState,

    tab_state: PaneGridState<InTabPane>,
    portrait_and_names: HashMap<String, (iced::widget::image::Handle, String)>,
    style_id_uuid_table: BTreeMap<i32, String>,
    icons: BTreeMap<i32, iced::widget::image::Handle>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TabContext {
    file_name: String,
    project: project::VoiceVoxProject,
    editing_line: usize,
}
impl Default for TabContext {
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
        Self {
            file_name: "unnamed".to_owned(),
            project: project::VoiceVoxProject {
                appVersion: "0.13.3".to_owned(),
                audioKeys: keys,
                audioItems: items,
            },
            editing_line: 0,
        }
    }
}
#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}
impl VoiceVoxState {
    fn get_tab_file_name(&self) -> Option<&str> {
        self.tabs
            .get(self.viewing_tab.unwrap_or_default())
            .map(|tab| tab.file_name.as_str())
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl VoiceVoxState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "Voiced")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("todos.json");

        path
    }

    async fn load() -> Result<VoiceVoxState, LoadError> {
        let mut contents = String::new();

        let mut file = tokio::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = tokio::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // This is a simple way to save at most once every couple seconds
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl VoiceVoxState {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;

        window.local_storage().ok()?
    }

    async fn load() -> Result<SavedState, LoadError> {
        let storage = Self::storage().ok_or(LoadError::File)?;

        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::File)?
            .ok_or(LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::File)?;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::Write)?;

        let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Main,
    ToolBarConfig,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum FileMenu {
    ExportAll,
    ExportSelected,
    ExportConnected,
    ExportTextConnected,
    ImportText,
    NewProject,
    SaveProject,
    SaveProjectAs,
    LoadProject,
}
impl std::fmt::Display for FileMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            FileMenu::ExportAll => "音声書き出し",
            FileMenu::ExportSelected => "一つだけ書き出し",
            FileMenu::ExportConnected => "音声を繋げて書き出し",
            FileMenu::ExportTextConnected => "テキストを繋げて書き出し",
            FileMenu::ImportText => "テキスト読み込み",
            FileMenu::NewProject => "新規プロジェクト",
            FileMenu::SaveProject => "プロジェクトを上書き保存",
            FileMenu::SaveProjectAs => "プロジェクトを名前を付けて保存",
            FileMenu::LoadProject => "プロジェクト読み込み",
        };
        write!(f, "{}", text)
    }
}
fn enumerate_file_menu() -> Vec<FileMenu> {
    use FileMenu::*;
    vec![
        ExportAll,
        ExportSelected,
        ExportConnected,
        ExportTextConnected,
        ImportText,
        NewProject,
        SaveProject,
        SaveProjectAs,
        LoadProject,
    ]
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum EngineMenu {
    Reboot,
}
impl std::fmt::Display for EngineMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "再起動")
    }
}
fn enumerate_engine_menu() -> Vec<EngineMenu> {
    vec![EngineMenu::Reboot]
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsMenu {
    KeyConfig,
    ToolbarConfig,
    ReorderCharacter,
    DefaultStyle,
    Dictionary,
    Option,
}
impl std::fmt::Display for SettingsMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            SettingsMenu::KeyConfig => "キー割り当て",
            SettingsMenu::ToolbarConfig => "ツールバーのカスタマイズ",
            SettingsMenu::ReorderCharacter => "キャラクターの並び替え・試聴",
            SettingsMenu::DefaultStyle => "デフォルトスタイル",
            SettingsMenu::Dictionary => "読み方&アクセント辞典",
            SettingsMenu::Option => "オプション",
        };
        write!(f, "{}", text)
    }
}
fn enumerate_settings_menu() -> Vec<SettingsMenu> {
    use SettingsMenu::*;
    vec![
        KeyConfig,
        ToolbarConfig,
        ReorderCharacter,
        DefaultStyle,
        Dictionary,
        Option,
    ]
}
