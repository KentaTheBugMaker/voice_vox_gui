use std::collections::{BTreeMap, HashMap};

use iced::{
    widget::{
        pane_grid::{State, TitleBar},
        Column, Row, Text,
    },
    Length, Renderer,
};
use iced_native::widget::pane_grid;

use crate::{history::History, toolbar::ToolBarConfig, Message, TabContext};
pub(crate) fn build_ui<'a>(
    tool_bar: &'a ToolBarConfig,

    in_tab_state: &'a State<InTabPane>,
    active_tab: usize,
    tab_contexts: &'a [TabContext],
    portraits: &HashMap<String, (iced::widget::image::Handle, String)>,
    style_id_uuid_table: &BTreeMap<i32, String>,
    icons: &BTreeMap<i32, iced::widget::image::Handle>,
    histories: &'a [History],
) -> Column<'a, Message, Renderer> {
    let mut page = Column::new();
    page = page.push(tool_bar.build_toolbar());

    let mut tab_bar =
        iced_aw::TabBar::new_without_right_click(active_tab, |id| Message::TabSelect(id));
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
                            let handle = tab_ctx
                                .project
                                .audioItems
                                .get(key)
                                .and_then(|audio_item| icons.get(&audio_item.styleId));
                            if let Some(handle) = handle {
                                line = line.push(iced::widget::button(
                                    iced::widget::image(handle.clone())
                                        .height(Length::Units(32))
                                        .width(Length::Units(32)),
                                ));
                            } else {
                                eprintln!(
                                    "Failed to find icon handle for line {}",
                                    tab_ctx.project.audioItems[key].styleId
                                );
                            }
                            // text
                            line = line.push(
                                iced::widget::text_input(
                                    "",
                                    &tab_ctx.project.audioItems.get(key).unwrap().text,
                                    |txt| Message::EditText(key.clone(), txt),
                                )
                                .on_submit(Message::QueryParameterCommit),
                            );
                            column = column.push(line);
                        }

                        pane_grid::Content::new(iced::widget::scrollable(
                            column.width(Length::Fill).height(Length::Fill),
                        ))
                    }
                    InTabPane::BottomPane => pane_grid::Content::new(Text::new("wip")),
                    InTabPane::ParameterPane => pane_grid::Content::new({
                        let mut column = Column::new();
                        let line = tab_ctx.editing_line;
                        let audio_item_key = &tab_ctx.project.audioKeys[line];
                        if let Some(audio_item) = tab_ctx.project.audioItems.get(audio_item_key) {
                            if let Some(query) = &audio_item.query {
                                column =
                                    column.push(Text::new(format!("話速 {:.2}", query.speedScale)));
                                column = column.push(
                                    iced::widget::slider(0.50..=2.0, query.speedScale, |x| {
                                        Message::SpeedChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                                column =
                                    column.push(Text::new(format!("音高 {:.2}", query.pitchScale)));
                                column = column.push(
                                    iced::widget::slider(-0.15..=0.15, query.pitchScale, |x| {
                                        Message::PitchChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                                column = column
                                    .push(Text::new(format!("抑揚 {:.2}", query.intonationScale)));
                                column = column.push(
                                    iced::widget::slider(0.0..=2.0, query.intonationScale, |x| {
                                        Message::IntonationChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                                column = column
                                    .push(Text::new(format!("音量 {:.2}", query.volumeScale)));
                                column = column.push(
                                    iced::widget::slider(0.0..=2.0, query.volumeScale, |x| {
                                        Message::VolumeChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                                column = column.push(Text::new(format!(
                                    "開始無音 {:.2}",
                                    query.prePhonemeLength
                                )));
                                column = column.push(
                                    iced::widget::slider(0.0..=1.5, query.prePhonemeLength, |x| {
                                        Message::PrePhonemeLengthChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                                column = column.push(Text::new(format!(
                                    "終了無音 {:.2}",
                                    query.postPhonemeLength
                                )));
                                column = column.push(
                                    iced::widget::slider(0.0..=1.5, query.postPhonemeLength, |x| {
                                        Message::PostPhonemeLengthChange(audio_item_key.clone(), x)
                                    })
                                    .step(0.001)
                                    .on_release(Message::QueryParameterCommit),
                                );
                            }
                        }

                        column
                    }),
                    InTabPane::CharacterPane => {
                        let line = tab_ctx.editing_line;
                        let audio_item_key = &tab_ctx.project.audioKeys[line];
                        let audio_item = tab_ctx.project.audioItems.get(audio_item_key);
                        let handle = audio_item
                            .and_then(|ai| style_id_uuid_table.get(&ai.styleId))
                            .and_then(|character_uuid| portraits.get(character_uuid));
                        if let Some(handle) = handle {
                            pane_grid::Content::new(iced::widget::image(handle.0.clone()))
                                .title_bar(TitleBar::new(Text::new(handle.1.clone())))
                        } else {
                            pane_grid::Content::new(Text::new("バグ"))
                        }
                    }
                    InTabPane::HistroyPane => {
                        let mut content = Column::new();
                        content = content.push("履歴");
                        if let Some(history) = histories.get(active_tab) {
                            content = content.push(iced::widget::scrollable(
                                history.build_view().width(Length::Fill),
                            ));
                        }
                        pane_grid::Content::new(content)
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
    HistroyPane,
}
