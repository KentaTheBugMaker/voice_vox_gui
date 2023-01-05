use iced::{
    alignment::{Horizontal, Vertical},
    widget::{self, button, column, row, tooltip::Position, Button, Row, Text},
    Length, Renderer,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToolBarKind {
    #[default]
    ContinuosPlay,
    Stop,
    ExportSelected,
    ExportAll,
    ConnectExport,
    SaveProject,
    Undo,
    Redo,
    LoadText,
    Blank,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolBarConfig(Vec<ToolBarKind>);
impl Default for ToolBarConfig {
    fn default() -> Self {
        Self(vec![
            ToolBarKind::ContinuosPlay,
            ToolBarKind::Stop,
            ToolBarKind::ExportSelected,
            ToolBarKind::Blank,
            ToolBarKind::Undo,
            ToolBarKind::Redo,
        ])
    }
}

impl ToolBarConfig {
    pub(crate) fn build_toolbar(&self) -> iced::widget::Row<crate::Message, iced::Renderer> {
        let mut toolbar = Row::new();

        for kind in self.0.iter() {
            let message = crate::Message::ToolBar(*kind);
            let button_name = kind.tool_name();
            let button = Button::new(
                Text::new(button_name)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            );
            if ToolBarKind::Blank == *kind {
                toolbar = toolbar.push(widget::horizontal_space(Length::Fill));
            } else {
                toolbar = toolbar.push(button.on_press(message));
            };
        }
        toolbar
    }

    pub(crate) fn build_toolbar_custom_ui(
        &self,
    ) -> iced::widget::Row<crate::Message, iced::Renderer> {
        let mut toolbar = Row::new();
        let mut space_exists = false;
        for kind in self.0.iter() {
            let message = crate::Message::ToolBarConfig(ConfigureMessage::Select(*kind));
            let button_name = kind.tool_name();
            let button = Button::new(
                Text::new(button_name)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            );
            let button = if ToolBarKind::Blank == *kind {
                button.width(Length::Fill)
            } else {
                button
            };

            let tooltip_text = kind.tooltip_text();
            let tooltip = widget::tooltip(
                button.on_press(message),
                tooltip_text,
                if space_exists {
                    Position::Left
                } else {
                    Position::Right
                },
            );

            if ToolBarKind::Blank == *kind {
                space_exists = true;
            }
            toolbar = toolbar.push(tooltip);
        }
        toolbar
    }
    pub(crate) fn insert(&mut self, kind: ToolBarKind) {
        self.0.push(kind)
    }
    pub(crate) fn remove(&mut self, kind: ToolBarKind) {
        self.0.retain(|k| *k != kind)
    }
    pub(crate) fn move_left(&mut self, kind: ToolBarKind) {
        let index = self.0.iter().enumerate().find(|k| *k.1 == kind);
        if let Some((index, _)) = index {
            if index > 0 {
                self.0.swap(index - 1, index);
            }
        }
    }
    pub(crate) fn move_right(&mut self, kind: ToolBarKind) {
        let index = self.0.iter().enumerate().find(|k| *k.1 == kind);
        if let Some((index, _)) = index {
            if !self.0.is_empty() && index < self.0.len() - 1 {
                self.0.swap(index, index + 1);
            }
        }
    }
}

impl ToolBarKind {
    fn enumerate() -> Vec<ToolBarKind> {
        vec![
            ToolBarKind::ContinuosPlay,
            ToolBarKind::Stop,
            ToolBarKind::ExportSelected,
            ToolBarKind::ExportAll,
            ToolBarKind::ConnectExport,
            ToolBarKind::SaveProject,
            ToolBarKind::Undo,
            ToolBarKind::Redo,
            ToolBarKind::LoadText,
            ToolBarKind::Blank,
        ]
    }
    fn tool_name(self) -> &'static str {
        match self {
            ToolBarKind::ContinuosPlay => "連続再生",
            ToolBarKind::Stop => "停止",
            ToolBarKind::ExportSelected => "1つ書き出し",
            ToolBarKind::ExportAll => "全部書き出し",
            ToolBarKind::ConnectExport => "つなげて書き出し",
            ToolBarKind::SaveProject => "プロジェクトを保存",
            ToolBarKind::Undo => "元に戻す",
            ToolBarKind::Redo => "やり直す",
            ToolBarKind::LoadText => "テキスト読み込み",
            ToolBarKind::Blank => "空白",
        }
    }
    fn tooltip_text(self) -> &'static str {
        ""
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigureMessage {
    Toggle(ToolBarKind, bool),
    RestoreDefault,
    Save,
    Exit,
    MoveLeft,
    MoveRight,
    Remove,
    Select(ToolBarKind),
}

pub(crate) fn build_configure_ui<'a>(
    config: &'a ToolBarConfig,
    selected_tool: &'a ToolBarKind,
) -> widget::Column<'a, crate::Message, Renderer> {
    let mut page = widget::Column::new()
        .push(row(vec![
            Text::new("ツールバーのカスタマイズ").into(),
            widget::horizontal_space(Length::Fill).into(),
            button(Text::new("デフォルトに戻す"))
                .on_press(crate::Message::ToolBarConfig(
                    ConfigureMessage::RestoreDefault,
                ))
                .into(),
            button(Text::new("保存"))
                .on_press(crate::Message::ToolBarConfig(ConfigureMessage::Save))
                .into(),
            button(Text::new("X"))
                .on_press(crate::Message::ToolBarConfig(ConfigureMessage::Exit))
                .into(),
        ]))
        .push(config.build_toolbar_custom_ui())
        .push(row(vec![
            Text::new(format!("「{}」を選択中", selected_tool.tool_name())).into(),
            widget::horizontal_space(Length::Fill).into(),
            button(Text::new("左に動かす"))
                .on_press(crate::Message::ToolBarConfig(ConfigureMessage::MoveLeft))
                .into(),
            button(Text::new("右に動かす"))
                .on_press(crate::Message::ToolBarConfig(ConfigureMessage::MoveRight))
                .into(),
            button(Text::new("削除"))
                .on_press(crate::Message::ToolBarConfig(ConfigureMessage::Remove))
                .into(),
        ]))
        .push(Text::new("表示するボタンの選択"));
    for kind in ToolBarKind::enumerate() {
        let contains = config.0.contains(&kind);
        let tool_name = kind.tool_name();
        let tooltip_text = kind.tooltip_text();
        let toggler = widget::toggler(None, contains, move |x| {
            crate::Message::ToolBarConfig(ConfigureMessage::Toggle(kind, x))
        })
        .width(Length::Fill);
        page = page.push(
            row(vec![
                column(vec![
                    Text::new(tool_name).into(),
                    Text::new(tooltip_text).into(),
                ])
                .width(Length::Fill)
                .into(),
                widget::horizontal_space(Length::Fill).into(),
                toggler.into(),
            ])
            .width(Length::Fill),
        );
    }
    page
}
