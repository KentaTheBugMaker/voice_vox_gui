use crate::Message;
use iced::{Element, Length, Renderer};
use iced_aw::menu::MenuTree;

pub(crate) fn build_character_change_button<'a>(
    menu: crate::OptionsRef<'a>,
    icon: iced_native::image::Handle,
) -> MenuTree<'a, Message, Renderer> {
    iced_aw::menu::MenuTree::new(
        iced::widget::button(iced::widget::image(icon))
            .height(Length::Fixed(32.0))
            .width(Length::Fixed(32.0)),
    )
    .into()
}
