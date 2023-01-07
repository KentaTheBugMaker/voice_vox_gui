use std::collections::{BTreeMap, HashMap};

use iced::{
    widget::{
        pane_grid::{State, TitleBar},
        Column, Row, Text,
    },
    Length, Renderer,
};
use iced_native::widget::pane_grid;

use crate::{toolbar::ToolBarConfig, Message, TabContext};
pub(crate) fn build_ui<'a>(
    tool_bar: &'a ToolBarConfig,

    in_tab_state: &'a State<InTabPane>,
    active_tab: usize,
    tab_contexts: &'a [TabContext],
    portraits: &HashMap<String, (iced::widget::image::Handle, String)>,
    style_id_uuid_table: &BTreeMap<i32, String>,
    icons: &BTreeMap<i32, iced::widget::image::Handle>,
) -> Column<'a, Message, Renderer> {
    let mut page = Column::new();
    page = page.push(tool_bar.build_toolbar());

    let mut tab_bar = iced_aw::TabBar::new(active_tab, |id| Message::TabSelect(id));
    for tab_ctx in tab_contexts {
        tab_bar = tab_bar.push(iced_aw::TabLabel::Text(tab_ctx.file_name.clone()));
    }
    if let Some(tab_ctx) = tab_contexts.get(active_tab) {
        let pane_grid =
            pane_grid::PaneGrid::new(
                in_tab_state,
                |_, intab_pane_kind, _| match intab_pane_kind {
                    InTabPane::TextPane => {
                        let mut column = Column::new();

                        for key in tab_ctx.project.audioKeys.iter() {
                            let mut line = Row::new();
                            // icon
                            let style_id = tab_ctx.project.audioItems[key].styleId;
                            let handle = icons[&style_id].clone();
                            line = line.push(iced::widget::button(
                                iced::widget::image(handle)
                                    .height(Length::Units(32))
                                    .width(Length::Units(32)),
                            ));
                            // text
                            line = line.push(iced::widget::text_input(
                                "",
                                &tab_ctx.project.audioItems.get(key).unwrap().text,
                                |txt| Message::EditText(key.clone(), txt),
                            ));
                            column = column.push(line);
                        }

                        pane_grid::Content::new(iced::widget::scrollable(
                            column.width(Length::Fill).height(Length::Fill),
                        ))
                    }
                    InTabPane::BottomPane => pane_grid::Content::new(Text::new("wip")),
                    InTabPane::ParameterPane => pane_grid::Content::new(Text::new("wip")),
                    InTabPane::CharacterPane => {
                        let line = tab_ctx.editing_line;
                        let audio_item_key = &tab_ctx.project.audioKeys[line];
                        let audio_item = tab_ctx.project.audioItems.get(audio_item_key);
                        if let Some(ai) = audio_item {
                            let character_uuid = &style_id_uuid_table[&ai.styleId];
                            let handle = portraits[character_uuid].clone();
                            pane_grid::Content::new(iced::widget::image(handle.0))
                                .title_bar(TitleBar::new(Text::new(handle.1)))
                        } else {
                            pane_grid::Content::new(Text::new("バグ"))
                        }
                    }
                },
            )
            .on_resize(10, crate::Message::IntabPaneResize)
            .width(Length::Fill)
            .height(Length::Fill);
        page.push(tab_bar.on_close(Message::TabClose))
            .push(pane_grid)
    } else {
        page.push(tab_bar.on_close(Message::TabClose))
    }
}

pub enum InTabPane {
    CharacterPane,
    TextPane,
    ParameterPane,
    BottomPane,
}
