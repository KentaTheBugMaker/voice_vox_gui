use std::collections::BTreeMap;

use iced::{
    overlay::menu::StyleSheet as MenuSS, widget::button::StyleSheet as ButtonSS,
    widget::container::StyleSheet as ContainerSS, Color, Length, Rectangle, Size,
};
use iced_native::{
    image,
    layout::Node,
    renderer::Quad,
    text,
    widget::{self, Tree},
    Element, Overlay, Widget,
};

struct CharacterChangeButton<'a, Message> {
    icon_table: &'a BTreeMap<String, iced_native::image::Handle>,
    name: &'a str,
    icon_size: (f32, f32),
    message: Box<dyn FnOnce(String) -> Message>,
}
impl<'a, Renderer, Message> Widget<Message, Renderer> for CharacterChangeButton<'a, Message>
where
    Renderer: iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>
        + iced_native::Renderer,
    Renderer::Theme: ButtonSS<Style = iced_native::renderer::Style>
        + ContainerSS<Style = iced_native::renderer::Style>+MenuSS<Style=iced_native::renderer::Style>,
{
    fn state(&self) -> iced_native::widget::tree::State {
        iced_native::widget::tree::State::new(CharacterChangeButtonState {
            open: false,
            sub_menu_hovered: None,
            menu: State::new(),
        })
    }

    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        Node::new(limits.resolve(Size {
            width: self.icon_size.0 + 3.0,
            height: self.icon_size.1 + 3.0,
        }))
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        _cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border_radius: 3.0.into(),
                border_width: 1.0,
                border_color: theme.active(style).border_color,
            },
            Color::WHITE,
        );
        if let Some(icon) = self.icon_table.get(self.name) {
            renderer.draw(
                icon.clone(),
                Rectangle::new(
                    iced::Point {
                        x: layout.position().x + 3.0,
                        y: layout.position().y + 3.0,
                    },
                    Size {
                        width: self.icon_size.0,
                        height: self.icon_size.1,
                    },
                ),
            );
        }
    }
    fn overlay<'b>(
        &'b mut self,
        state: &'b mut iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        let state = state.state.downcast_mut::<CharacterChangeButtonState>();
        if state.open {
            let overlay_position = layout.position();
            Some(MenuOverlay::new(&mut state.menu,layout.bounds().height).overlay(overlay_position))
        } else {
            None
        }
    }
    fn on_event(
        &mut self,
        state: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _ime: &dyn iced_native::IME,
        _shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        let state = state.state.downcast_mut::<CharacterChangeButtonState>();
        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left))
            | iced::Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();
                state.open = bounds.contains(cursor_position);
                iced::event::Status::Captured
            }
            _ => iced::event::Status::Ignored,
        }
    }
}
struct MenuOverlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>,
    Renderer::Theme: ContainerSS + ButtonSS+MenuSS,
{
    state: &'a mut Tree,
    row_height: f32,
    icon_width: f32,
    container: iced_native::widget::Container<'a, Message, Renderer>,
    style: <Renderer::Theme as MenuSS>::Style,
}

struct Menu<'a, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: MenuSS,
{
    state: &'a mut State,
    hovered: &'a mut Option<usize>,
    last_selection: &'a mut Option<String>,
    width: u16,
    text_size: Option<u16>,
    font: Renderer::Font,
    style: <Renderer::Theme as MenuSS>::Style,
}

impl<'a, Message, Renderer> MenuOverlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>,
    Renderer::Theme:ButtonSS<Style = iced_native::renderer::Style>
    + ContainerSS<Style = iced_native::renderer::Style>+MenuSS<Style=iced_native::renderer::Style>,
{
    fn new(menu:Menu<'a,Renderer>,target_height:f32) -> Self {
        let Menu{ state, hovered, last_selection, width, text_size,  font, style }=menu;
        // build overlay ui.
        let container = widget::container(widget::scrollable(widget::column(vec![])));
        Self {
            state: &mut state.tree,
            row_height: (),
            icon_width: (),
            container: (),
            style: (),
        }
    }
}

struct State {
    tree: Tree,
}
impl State {
    /// Creates a new [`State`] for a [`Menu`].
    pub fn new() -> Self {
        Self {
            tree: Tree::empty(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, Message, Renderer> Overlay<Message, Renderer> for MenuOverlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>,
    Renderer::Theme: ContainerSS+ButtonSS+MenuSS
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: Size,
        position: iced::Point,
    ) -> iced_native::layout::Node {
        // resolve height,
        let text_size = self.row_height;

        todo!()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
    ) {
        todo!()
    }
}
impl<'a, Message, Renderer> MenuOverlay<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::renderer::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = iced_native::image::Handle>
        + 'a,
    Renderer::Theme:ContainerSS+ButtonSS+MenuSS
{
    fn overlay(self, point: iced::Point) -> iced_native::overlay::Element<'a, Message, Renderer> {
        iced_native::overlay::Element::new(point, Box::new(self))
    }
}

impl<'a, Message, Renderer> From<CharacterChangeButton<'a, Message>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'a + image::Renderer<Handle = iced_native::image::Handle>,
    Renderer::Theme:ButtonSS<Style = iced_native::renderer::Style>
    + ContainerSS<Style = iced_native::renderer::Style>+MenuSS<Style=iced_native::renderer::Style>,
{
    fn from(pick_list: CharacterChangeButton<'a, Message>) -> Self {
        Self::new(pick_list)
    }
}
struct CharacterChangeButtonState {
    open: bool,
    sub_menu_hovered: Option<String>,
    menu:State,
}
