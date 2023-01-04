use iced::{
    widget::{self, Button, Row, Text},
    Length,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ToolBarKind {
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
            if let ToolBarKind::Blank = kind {
                toolbar = toolbar.push(widget::horizontal_space(Length::Fill));
            } else {
                let message = crate::Message::ToolBar(*kind);
                let content = match kind {
                    ToolBarKind::ContinuosPlay => Button::new(Text::new("連続再生")),
                    ToolBarKind::Stop => Button::new(Text::new("停止")),
                    ToolBarKind::ExportSelected => Button::new(Text::new("1つ書き出し")),
                    ToolBarKind::ExportAll => Button::new(Text::new("全部書き出し")),
                    ToolBarKind::ConnectExport => Button::new(Text::new("つなげて書き出し")),
                    ToolBarKind::SaveProject => Button::new(Text::new("プロジェクトを保存")),
                    ToolBarKind::Undo => Button::new(Text::new("元に戻す")),
                    ToolBarKind::Redo => Button::new(Text::new("やり直す")),
                    ToolBarKind::LoadText => Button::new(Text::new("テキスト読み込み")),
                    _ => unreachable!("Don't make button for blank"),
                };
                toolbar = toolbar.push(content.on_press(message));
            }
        }
        toolbar
    }

    pub(crate) fn build_toolbar_custom_ui(
        &self,
    ) -> iced::widget::Row<crate::Message, iced::Renderer> {
        let mut toolbar = Row::new();
        for kind in self.0.iter() {
            let message = crate::Message::ToolBar(*kind);
            let content = match kind {
                ToolBarKind::ContinuosPlay => Button::new(Text::new("連続再生")),
                ToolBarKind::Stop => Button::new(Text::new("停止")),
                ToolBarKind::ExportSelected => Button::new(Text::new("1つ書き出し")),
                ToolBarKind::ExportAll => Button::new(Text::new("全部書き出し")),
                ToolBarKind::ConnectExport => Button::new(Text::new("つなげて書き出し")),
                ToolBarKind::SaveProject => Button::new(Text::new("プロジェクトを保存")),
                ToolBarKind::Undo => Button::new(Text::new("元に戻す")),
                ToolBarKind::Redo => Button::new(Text::new("やり直す")),
                ToolBarKind::LoadText => Button::new(Text::new("テキスト読み込み")),
                ToolBarKind::Blank => Button::new(Text::new("空白")).width(Length::Fill),
            };
            toolbar = toolbar.push(content.on_press(message));
        }
        toolbar
    }
}
