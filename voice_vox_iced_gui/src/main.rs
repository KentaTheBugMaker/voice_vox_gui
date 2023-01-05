mod toolbar;

use toolbar::{build_configure_ui, ConfigureMessage, ToolBarConfig, ToolBarKind};

use iced::{
    widget::{self, column, Button, Column, PickList, Row, Text},
    Application, Command, Element, Settings, Theme,
};
use serde::{Deserialize, Serialize};

fn main() -> iced::Result {
    VoiceVox::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansCJKjp-Regular.otf")),
        ..Default::default()
    })
}

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
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Self::Loaded(State {
                            dirty: false,
                            saving: false,
                            persistence: state,
                            opening_page: Page::ToolBarConfig,
                            configure_ui_selected_tool: ToolBarKind::default(),
                            toolbar_ui_temp_config: ToolBarConfig::default(),
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Self::Loaded(State::default());
                    }
                    _ => {}
                }
                Command::none()
            }
            Self::Loaded(state) => {
                let mut saved = false;

                let command = Command::none();

                match message {
                    Message::FileMenuOpen(file_menu) => {}
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

                Command::batch(vec![command, save])
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
                    let mut tab_bar = Row::new();

                    column(vec![
                        state.persistence.tool_bar_config.build_toolbar().into(),
                        tab_bar.into(),
                    ])
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
#[derive(Debug, Default)]
struct State {
    dirty: bool,
    saving: bool,
    opening_page: Page,
    configure_ui_selected_tool: ToolBarKind,
    toolbar_ui_temp_config: ToolBarConfig,
    persistence: VoiceVoxState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TabContext {
    file_name: String,
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
        use async_std::prelude::*;

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
        use async_std::prelude::*;

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
