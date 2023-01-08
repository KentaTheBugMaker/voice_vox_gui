use std::hash::Hash;

use iced::{event, Color, Element, Length, Rectangle};
use iced_native::{
    overlay,
    renderer::{BorderRadius, Quad},
};

pub enum ChildDirection {
    Right,
    Bottom,
}

pub struct MenuContainer<Handle, Renderer>
where
    Handle: Clone + Hash,
    Renderer: iced_native::text::Renderer,
{
    children: Vec<ButtonOrSeparator<Handle, Renderer>>,
    direction: ChildDirection,
}

enum ButtonOrSeparator<Handle: Clone + Hash, Renderer: iced_native::text::Renderer> {
    Separator,
    Menu(MenuButton<Handle, Renderer>),
}

pub struct MenuButton<Handle, Renderer>
where
    Handle: Clone + Hash,
    Renderer: iced_native::text::Renderer,
{
    sub_menu: MenuContainer<Handle, Renderer>,
    handle: Handle,
    text: String,
    font: Renderer::Font,
    size: Option<u16>,
    padding: u16,
}
impl<Message, Renderer, Handle> iced_native::Widget<Message, Renderer>
    for MenuContainer<Handle, Renderer>
where
    Handle: Clone + Hash,
    Renderer: iced_native::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = Handle>,
{
    fn width(&self) -> iced::Length {
        match self.direction {
            ChildDirection::Right => Length::Fill,
            ChildDirection::Bottom => Length::Shrink,
        }
    }

    fn height(&self) -> iced::Length {
        match self.direction {
            ChildDirection::Right => Length::Shrink,
            ChildDirection::Bottom => Length::Shrink,
        }
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let width = match self.direction {
            ChildDirection::Right => Length::Fill,
            ChildDirection::Bottom => Length::Shrink,
        };
        let height = match self.direction {
            ChildDirection::Right => Length::Shrink,
            ChildDirection::Bottom => Length::Shrink,
        };
        let size = limits.height(height).width(width).max();
        iced_native::layout::Node::new(size)
    }

    fn draw(
        &self,
        tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            match child {
                ButtonOrSeparator::Separator => {
                    match self.direction {
                        ChildDirection::Right => {
                            //vertical line
                            renderer.fill_quad(
                                Quad {
                                    bounds: Rectangle::default(),
                                    border_radius: BorderRadius::default(),
                                    border_width: 0.0,
                                    border_color: Color::default(),
                                },
                                Color::BLACK,
                            )
                        }
                        ChildDirection::Bottom => {
                            // horizontal line
                            renderer.fill_quad(
                                Quad {
                                    bounds: Rectangle::default(),
                                    border_radius: BorderRadius::default(),
                                    border_width: 0.0,
                                    border_color: Color::default(),
                                },
                                Color::BLACK,
                            )
                        }
                    }
                }
                ButtonOrSeparator::Menu(child) => {
                    iced_native::Widget::<Message, Renderer>::draw(
                        child,
                        state,
                        renderer,
                        theme,
                        style,
                        layout,
                        cursor_position,
                        viewport,
                    );
                }
            }
        }
    }
    fn on_event(
        &mut self,
        tree: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        ime: &dyn iced_native::IME,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                if let ButtonOrSeparator::Menu(child) = child {
                    child.on_event(
                        state,
                        event.clone(),
                        layout,
                        cursor_position,
                        renderer,
                        clipboard,
                        ime,
                        shell,
                    )
                } else {
                    event::Status::Ignored
                }
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }
}
impl<Handle, Renderer> MenuContainer<Handle, Renderer>
where
    Handle: Clone + Hash,
    Renderer: iced_native::text::Renderer,
{
    pub fn new() -> Self {
        Self {
            children: vec![],
            direction: ChildDirection::Right,
        }
    }
}
impl<'a, Message, Renderer, Handle> From<MenuContainer<Handle, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_native::renderer::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = Handle>
        + 'a,
    Handle: 'a + Clone + Hash,
{
    fn from(container: MenuContainer<Handle, Renderer>) -> Self {
        Self::new(container)
    }
}
impl<'a, Message, Renderer, Handle> iced_native::Widget<Message, Renderer>
    for MenuButton<Handle, Renderer>
where
    Renderer: iced_native::renderer::Renderer
        + iced_native::text::Renderer
        + iced_native::image::Renderer<Handle = Handle>,
    Handle: 'a + Clone + Hash,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let image_dimension = renderer.dimensions(&self.handle);
        let (text_height, text_width) = renderer.measure(
            &self.text,
            self.size.unwrap_or(renderer.default_size()),
            self.font.clone(),
            bounds.size(),
        );
        let scaling = image_dimension.height as f32 / text_height;
        let image_width = image_dimension.width as f32 / scaling;
        // render
        // image | padding| text
        let image_position = bounds.position();
        let image_size = iced::Size {
            width: image_width,
            height: text_height,
        };
        let image_bounds = Rectangle::new(image_position, image_size);
        renderer.draw(self.handle.clone(), image_bounds);
        let text_position = bounds.position();
        let text_position = iced::Point::new(
            text_position.x + image_width + self.padding as f32,
            text_position.y,
        );
        let size = iced::Size {
            width: text_width,
            height: text_height,
        };
        let text_bounds = Rectangle::new(text_position, size);
        renderer.fill_text(iced_native::text::Text {
            content: &self.text,
            bounds: text_bounds,
            size: self.size.unwrap_or(renderer.default_size()) as f32,
            color: Color::default(),
            font: self.font.clone(),
            horizontal_alignment: iced::alignment::Horizontal::Left,
            vertical_alignment: iced::alignment::Vertical::Center,
        });
    }
}
