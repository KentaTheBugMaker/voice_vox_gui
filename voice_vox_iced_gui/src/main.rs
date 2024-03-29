mod character_change_button;
mod history;
mod main_page;
mod project;
mod toolbar;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use async_std::io::{ReadExt, WriteExt};
use history::{Diff, History};
use iced::widget::pane_grid::{self, State as PaneGridState};
use iced::{
    widget::{self, column, Button, Column, PickList, Row, Text},
    Application, Command, Element, Settings, Theme,
};
use main_page::InTabPane;

use project::VoiceVoxProject;
use serde::{Deserialize, Serialize};

use toolbar::{build_configure_ui, ConfigureMessage, ToolBarConfig, ToolBarKind};
use voice_vox_api::api::{APIError, MorpableTargets, SpeakerInfo};

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
    SpeedChange(String, f64),
    PitchChange(String, f64),
    IntonationChange(String, f64),
    VolumeChange(String, f64),
    PrePhonemeLengthChange(String, f64),
    PostPhonemeLengthChange(String, f64),
    QueryParameterCommit,
    APICall(APICall),
    APIResult(APIResult),
    CharacterChange(String, i32),
    FileLoadError,
    NewTab(TabContext),
    NewAudioCell,
}
#[derive(Debug, Clone)]
pub(crate) enum APIResult {
    Speakers(Result<Vec<voice_vox_api::api_schema::Speaker>, APIError>),
    SpeakerInfo(
        voice_vox_api::api_schema::Speaker,
        Result<voice_vox_api::api_schema::SpeakerInfo, APIError>,
    ),
    MorpableTargets(
        i32,
        Result<Vec<HashMap<i32, voice_vox_api::api_schema::MorphableTargetInfo>>, APIError>,
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
                    ratio: 0.4,
                    a: Box::new(pane_grid::Configuration::Split {
                        axis: pane_grid::Axis::Vertical,
                        ratio: 0.2,
                        a: Box::new(pane_grid::Configuration::Pane(InTabPane::Character)),
                        b: Box::new(pane_grid::Configuration::Split {
                            axis: pane_grid::Axis::Vertical,
                            ratio: 0.5,
                            a: Box::new(pane_grid::Configuration::Pane(InTabPane::Text)),
                            b: Box::new(pane_grid::Configuration::Pane(InTabPane::Parameter)),
                        }),
                    }),
                    b: Box::new(pane_grid::Configuration::Split {
                        axis: pane_grid::Axis::Vertical,
                        ratio: 0.5,
                        a: Box::new(pane_grid::Configuration::Pane(InTabPane::Bottom)),
                        b: Box::new(pane_grid::Configuration::Pane(InTabPane::History)),
                    }),
                };
                match message {
                    Message::Loaded(Ok(state)) => {
                        let buffer_count = state.tabs.len();
                        *self = Self::Loaded(State {
                            dirty: false,
                            saving: false,
                            persistence: state,
                            opening_page: Page::Main,
                            configure_ui_selected_tool: ToolBarKind::default(),
                            toolbar_ui_temp_config: ToolBarConfig::default(),

                            tab_state: PaneGridState::with_configuration(configure),
                            portrait_and_names: BTreeMap::new(),
                            style_id_uuid_table: BTreeMap::new(),
                            tracking_buffer: vec![History::new(); buffer_count],
                            character_change_menu: vec![],
                            prev_style_id_table_len: 0,
                            morphable_targets: BTreeMap::new(),
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Self::Loaded(State {
                            dirty: false,
                            saving: false,
                            persistence: VoiceVoxState::default(),
                            opening_page: Page::Main,
                            configure_ui_selected_tool: ToolBarKind::default(),
                            toolbar_ui_temp_config: ToolBarConfig::default(),

                            tab_state: PaneGridState::with_configuration(configure),
                            portrait_and_names: BTreeMap::new(),
                            style_id_uuid_table: BTreeMap::new(),
                            tracking_buffer: Vec::new(),
                            character_change_menu: vec![],
                            prev_style_id_table_len: 0,
                            morphable_targets: BTreeMap::new(),
                        });
                    }
                    _ => {}
                }
                // collect informations from engine.s
                Command::perform(
                    voice_vox_api::api::Speakers { core_version: None }.call("localhost:50021"),
                    |res| Message::APIResult(APIResult::Speakers(res)),
                )
            }
            Self::Loaded(state) => {
                let mut saved = false;
                let mut cmd_buff = vec![];
                if state.prev_style_id_table_len != state.style_id_uuid_table.len() {
                    state.character_change_menu = build_character_change_menu(
                        &state.portrait_and_names,
                        &state.style_id_uuid_table,
                    );
                    state.prev_style_id_table_len = state.style_id_uuid_table.len();
                }
                match message {
                    Message::FileMenuOpen(file_menu) => match file_menu {
                        FileMenu::ExportAll => todo!(),
                        FileMenu::ExportSelected => todo!(),
                        FileMenu::ExportConnected => todo!(),
                        FileMenu::ExportTextConnected => todo!(),
                        FileMenu::ImportText => todo!(),
                        FileMenu::NewProject => {
                            state.persistence.tabs.push(TabContext::default());
                            if !state.persistence.tabs.is_empty() {
                                state.persistence.viewing_tab =
                                    Some(state.persistence.tabs.len() - 1);
                            }
                            state.tracking_buffer.push(History::new());
                        }

                        FileMenu::SaveProject => todo!(),
                        FileMenu::SaveProjectAs => todo!(),
                        FileMenu::LoadProject => cmd_buff.push(Command::perform(
                            rfd::AsyncFileDialog::new()
                                .add_filter("VoiceVox project file", &["vvproj"])
                                .pick_file(),
                            |filehandle| {
                                if let Some(file_handle) = filehandle {
                                    let path = file_handle.path();
                                    let data: Result<VoiceVoxProject, Message> =
                                        std::fs::read(path)
                                            .map_err(|_| Message::FileLoadError)
                                            .and_then(|data| {
                                                serde_json::from_slice(&data)
                                                    .map_err(|_| Message::FileLoadError)
                                            });

                                    if let Ok(data) = data {
                                        Message::NewTab(TabContext {
                                            file_name: file_handle.file_name(),
                                            project: data,
                                            editing_line: 0,
                                        })
                                    } else {
                                        Message::FileLoadError
                                    }
                                } else {
                                    Message::FileLoadError
                                }
                            },
                        )),
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
                    Message::ToolBar(tbk) => match tbk {
                        ToolBarKind::ContinuosPlay => todo!(),
                        ToolBarKind::Stop => todo!(),
                        ToolBarKind::ExportSelected => todo!(),
                        ToolBarKind::ExportAll => todo!(),
                        ToolBarKind::ConnectExport => todo!(),
                        ToolBarKind::SaveProject => todo!(),
                        ToolBarKind::Undo => {
                            if let Some((history, tab_ctx)) =
                                state.persistence.viewing_tab.and_then(|tab_id| {
                                    state
                                        .tracking_buffer
                                        .get_mut(tab_id)
                                        .zip(state.persistence.tabs.get_mut(tab_id))
                                })
                            {
                                println!("undo");
                                history.undo(tab_ctx);
                            }
                        }
                        ToolBarKind::Redo => {
                            if let Some((history, tab_ctx)) =
                                state.persistence.viewing_tab.and_then(|tab_id| {
                                    state
                                        .tracking_buffer
                                        .get_mut(tab_id)
                                        .zip(state.persistence.tabs.get_mut(tab_id))
                                })
                            {
                                println!("redo ");
                                history.redo(tab_ctx);
                            }
                        }
                        ToolBarKind::LoadText => todo!(),
                        ToolBarKind::Blank => todo!(),
                    },
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
                        state.tracking_buffer.remove(tab_id);

                        let save = Command::perform(
                            async_std::fs::write(
                                tab_ctx.file_name,
                                serde_json::ser::to_vec_pretty(&tab_ctx.project).unwrap(),
                            ),
                            |x| Message::Saved(x.map_err(|_| SaveError::Write)),
                        );
                        cmd_buff.push(save);
                    }
                    Message::EditText(key, text) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::Text {
                                        audio_item_key: key,
                                        before: String::new(),
                                        after: text,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::APICall(_request) => {}
                    Message::APIResult(result) => match result {
                        APIResult::Speakers(speakers) => {
                            println!("{speakers:?}");

                            if let Ok(speakers) = speakers {
                                for speaker in speakers {
                                    cmd_buff.push(Command::perform(
                                        SpeakerInfo {
                                            speaker_uuid: speaker.speaker_uuid.clone(),
                                            core_version: None,
                                        }
                                        .call(SERVER.get().unwrap()),
                                        |response| {
                                            Message::APIResult(APIResult::SpeakerInfo(
                                                speaker, response,
                                            ))
                                        },
                                    ));
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
                                        speaker.styles.iter().map(|x| x.id).collect(),
                                    ),
                                );
                                for style in speaker.styles.iter() {
                                    let style_id = style.id;
                                    {
                                        cmd_buff.push(Command::perform(
                                            MorpableTargets {
                                                style_id: vec![style_id],
                                                core_version: None,
                                            }
                                            .call(SERVER.get().unwrap()),
                                            move |response| {
                                                Message::APIResult(APIResult::MorpableTargets(
                                                    style_id, response,
                                                ))
                                            },
                                        ))
                                    }
                                }
                                state
                                    .style_id_uuid_table
                                    .append(&mut speaker.styles.iter().fold(
                                        BTreeMap::new(),
                                        |mut buffer, speaker_style| {
                                            if let Some(style_info) =
                                                info.style_infos.iter().find(|style_info| {
                                                    style_info.id == speaker_style.id
                                                })
                                            {
                                                buffer.insert(
                                                    style_info.id,
                                                    (
                                                        speaker.speaker_uuid.clone(),
                                                        speaker_style.name.clone(),
                                                        widget::image::Handle::from_memory(
                                                            style_info.icon.clone(),
                                                        ),
                                                    ),
                                                );
                                                buffer
                                            } else {
                                                buffer
                                            }
                                        },
                                    ));
                            }
                        }
                        APIResult::MorpableTargets(style_id, morphable_targets) => {
                            if let Ok(mut morphable_targets) = morphable_targets {
                                state.morphable_targets.insert(
                                    style_id,
                                    morphable_targets[0]
                                        .drain()
                                        .filter(|(_, mti)| mti.is_morphable)
                                        .map(|(style_id, _)| style_id)
                                        .collect(),
                                );
                            }
                        }
                    },
                    Message::SpeedChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::Speed {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::PitchChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::Pitch {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::IntonationChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::Intonation {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::VolumeChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::Volume {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::PrePhonemeLengthChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::PrePhonemeLength {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::PostPhonemeLengthChange(key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::PostPhonemeLength {
                                        audio_item_key: key,
                                        before: 0.0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::QueryParameterCommit => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            state.tracking_buffer[vt].commit()
                        }
                    }
                    Message::CharacterChange(audio_item_key, after) => {
                        if let Some(vt) = state.persistence.viewing_tab {
                            if let Some(tab_ctx) = state.persistence.tabs.get_mut(vt) {
                                state.tracking_buffer[vt].apply(
                                    Diff::CharacterChange {
                                        audio_item_key,
                                        before: 0,
                                        after,
                                    },
                                    tab_ctx,
                                );
                            }
                        }
                    }
                    Message::FileLoadError => {}
                    Message::NewTab(tab_ctx) => {
                        state.persistence.tabs.push(tab_ctx);
                        state.tracking_buffer.push(History::new());
                    }
                    Message::NewAudioCell => {
                        if let Some(tab_ctx) = state
                            .persistence
                            .viewing_tab
                            .and_then(|tab_number| state.persistence.tabs.get_mut(tab_number))
                        {
                            tab_ctx.project.add_audio_cell();
                        }
                    }
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
                        &state.tracking_buffer,
                        &state.character_change_menu,
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
    tracking_buffer: Vec<History>,
    tab_state: PaneGridState<InTabPane>,
    /// UUID -> (Portrait,Name,StyleIDs)
    portrait_and_names: BTreeMap<String, (iced::widget::image::Handle, String, Vec<i32>)>,
    /// StyleID -> (UUID,StyleName,Icon)
    style_id_uuid_table: BTreeMap<i32, (String, String, iced::widget::image::Handle)>,
    /// (Name,stylemenu (icon ,stylename,styleId))
    character_change_menu: OptionsOwned,
    prev_style_id_table_len: usize,
    morphable_targets: BTreeMap<i32, BTreeSet<i32>>,
}
pub(crate) type OptionsOwned = Vec<(String, Vec<(iced::widget::image::Handle, String, i32)>)>;
pub(crate) type OptionsRef<'a> = &'a [(String, Vec<(iced::widget::image::Handle, String, i32)>)];
fn build_character_change_menu(
    portrait_and_names: &BTreeMap<String, (iced::widget::image::Handle, String, Vec<i32>)>,
    style_id_uuid_table: &BTreeMap<i32, (String, String, iced::widget::image::Handle)>,
) -> OptionsOwned {
    let mut menu = vec![];
    for (_, (_, name, style_ids)) in portrait_and_names.iter() {
        let mut sub_menu = vec![];
        for style_id in style_ids {
            if let Some((_, style_name, icon)) = style_id_uuid_table.get(style_id) {
                sub_menu.push((icon.clone(), style_name.clone(), *style_id));
            }
        }
        menu.push((name.clone(), sub_menu));
    }
    println!("built {menu:?}");
    menu
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TabContext {
    file_name: String,
    project: project::VoiceVoxProject,
    editing_line: usize,
}
impl Default for TabContext {
    fn default() -> Self {
        Self {
            file_name: "unnamed".to_owned(),
            project: project::VoiceVoxProject::default(),
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

        let mut file = async_std::fs::File::open(Self::path())
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
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

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
        write!(f, "{text}")
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
        write!(f, "{text}")
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
