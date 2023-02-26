use crate::Message;
use iced::{
    widget::{row, text},
    Color, Length, Renderer,
};
use iced_aw::menu::MenuTree;
use iced_native::widget::button;

pub(crate) fn build_character_change_button<'a>(
    menu: crate::OptionsRef<'a>,
    icon: iced_native::image::Handle,
    line_uuid: &'a str,
) -> MenuTree<'a, Message, Renderer> {
    let children = menu
        .iter()
        .map(|(character_name, styles)| {
            if styles.len() > 1 {
                let sub_menu = styles
                    .iter()
                    .map(|(icon, style_name, id)| {
                        MenuTree::new(
                            iced::widget::button(
                                row(vec![
                                    iced::widget::image(icon.clone())
                                        .height(Length::Fixed(32.0))
                                        .width(Length::Fixed(32.0))
                                        .into(),
                                    text(format!("{character_name:}({})", style_name)).into(),
                                ])
                                .align_items(iced::Alignment::Start),
                            )
                            .on_press(Message::CharacterChange(line_uuid.to_owned(), *id))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                            .width(Length::Fill),
                        )
                    })
                    .collect();
                MenuTree::with_children(
                    iced::widget::button(
                        row(vec![
                            iced::widget::image(styles[0].0.clone())
                                .height(Length::Fixed(32.0))
                                .width(Length::Fixed(32.0))
                                .into(),
                            iced::widget::text(character_name).into(),
                        ])
                        .align_items(iced::Alignment::Start),
                    )
                    .on_press(Message::CharacterChange(line_uuid.to_owned(), styles[0].2))
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                    .width(Length::Fill),
                    sub_menu,
                )
            } else {
                MenuTree::new(
                    iced::widget::button(
                        row(vec![
                            iced::widget::image(styles[0].0.clone())
                                .height(Length::Fixed(32.0))
                                .width(Length::Fixed(32.0))
                                .into(),
                            iced::widget::text(character_name).into(),
                        ])
                        .align_items(iced::Alignment::Start),
                    )
                    .on_press(Message::CharacterChange(line_uuid.to_owned(), styles[0].2))
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
                    .width(Length::Fill),
                )
            }
        })
        .collect();
    iced_aw::menu::MenuTree::with_children(
        iced::widget::button(iced::widget::image(icon))
            .height(Length::Fixed(32.0))
            .width(Length::Fixed(32.0))
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {}))),
        children,
    )
}

struct ButtonStyle;

impl iced_native::widget::button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            border_radius: 4.0,
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}
