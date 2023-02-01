//! Display a dropdown list of selectable values.
use std::borrow::Cow;

use iced::{
    event, keyboard, touch, widget::pick_list::StyleSheet, Event, Length, Padding, Point,
    Rectangle, Size,
};
use iced_native::{
    image, layout, mouse,
    overlay::{self, menu},
    renderer,
    text::{self},
    widget::{container, scrollable, tree, Tree},
    Clipboard, Element, Layout, Shell, Widget, IME,
};

use crate::character_change_dropdown_menu::Menu;
pub(crate) type OptionsCow<'a>= Cow<'a, [(String, Vec<(image::Handle, String, i32)>)]>;
/// A widget for selecting a single value from a list of options.
#[allow(missing_debug_implementations)]
pub struct CharacterChangeButton<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    on_selected: Box<dyn Fn(i32) -> Message + 'a>,
    options:OptionsCow<'a>,
    icon: Option<iced_native::image::Handle>,
    selected: Option<i32>,
    width: Length,
    padding: Padding,
    text_size: Option<u16>,
    font: Renderer::Font,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> CharacterChangeButton<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
    <Renderer::Theme as menu::StyleSheet>::Style: From<<Renderer::Theme as StyleSheet>::Style>,
{
    /// The default padding of a [`PickList`].
    pub const DEFAULT_PADDING: Padding = Padding::new(5);

    /// Creates a new [`PickList`] with the given list of options, the current
    /// selected value, and the message to produce when an option is selected.
    pub fn new(
        options: impl Into<OptionsCow<'a>>,
        selected: Option<i32>,
        on_selected: impl Fn(i32) -> Message + 'a,
    ) -> Self {
        Self {
            on_selected: Box::new(on_selected),
            options: options.into(),
            icon: None,
            selected,
            width: Length::Shrink,
            padding: Self::DEFAULT_PADDING,
            text_size: None,
            font: Default::default(),
            style: Default::default(),
        }
    }

    /// Sets the placeholder of the [`PickList`].
    pub fn icon(mut self, icon: iced_native::image::Handle) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the width of the [`PickList`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the [`Padding`] of the [`PickList`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`PickList`].
    pub fn text_size(mut self, size: u16) -> Self {
        self.text_size = Some(size);
        self
    }

    /// Sets the font of the [`PickList`].
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`PickList`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for CharacterChangeButton<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + image::Renderer<Handle = iced_native::image::Handle> + 'a,
    Renderer::Theme: StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
    <Renderer::Theme as menu::StyleSheet>::Style: From<<Renderer::Theme as StyleSheet>::Style>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let default_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        layout(
            limits,
            self.width,
            self.padding,
            Size::new(default_size as f32, default_size as f32),
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _ime: &dyn IME,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        update(
            event,
            layout,
            cursor_position,
            shell,
            self.on_selected.as_ref(),
            self.selected.as_ref(),
            &self.options,
            || tree.state.downcast_mut::<State>(),
        )
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor_position)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let placeholder = self.icon.clone().or_else(|| {
            self.options
                .iter()
                .find(|(_, styles)| {
                    styles
                        .iter()
                        .any(|(_, _, style_id)| self.selected == Some(*style_id))
                })
                .map(|(_, styles)| styles)
                .and_then(|styles| {
                    styles
                        .iter()
                        .find(|(_, _, style_id)| self.selected == Some(*style_id))
                        .map(|(icon, _, _)| icon.clone())
                })
        });
        draw(
            renderer,
            theme,
            layout,
            cursor_position,
            placeholder,
            &self.style,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let state = tree.state.downcast_mut::<State>();
        let default_size = self.text_size.unwrap_or_else(|| renderer.default_size());

        overlay(
            layout,
            state,
            self.padding,
            self.text_size,
            self.font.clone(),
            &self.options,
            self.style.clone(),
            Size::new(default_size as f32, default_size as f32),
        )
    }
}

impl<'a, Message, Renderer> From<CharacterChangeButton<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle> + 'a,
    Renderer::Theme: StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
    <Renderer::Theme as menu::StyleSheet>::Style: From<<Renderer::Theme as StyleSheet>::Style>,
{
    fn from(pick_list: CharacterChangeButton<'a, Message, Renderer>) -> Self {
        Self::new(pick_list)
    }
}

/// The local state of a [`PickList`].
#[derive(Debug)]
pub struct State {
    menu: crate::character_change_dropdown_menu::State,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    hovered_option: Option<usize>,
    hovered_notch: Option<usize>,
    last_selection: Option<i32>,
    style_menu: Option<usize>,
    style_menu_hovered_option: Option<usize>,
}

impl State {
    /// Creates a new [`State`] for a [`PickList`].
    pub fn new() -> Self {
        Self {
            menu: crate::character_change_dropdown_menu::State::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: bool::default(),
            hovered_option: Option::default(),
            last_selection: Option::default(),
            style_menu: Option::default(),
            style_menu_hovered_option: Option::default(),
            hovered_notch: Option::default(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the layout of a [`PickList`].
pub fn layout(
    limits: &layout::Limits,
    width: Length,
    padding: Padding,
    image_dimension: Size,
) -> layout::Node {
    let limits = limits.width(width).height(Length::Shrink).pad(padding);

    let size = {
        let intrinsic = image_dimension;

        limits.resolve(intrinsic).pad(padding)
    };

    layout::Node::new(size)
}

/// Processes an [`Event`] and updates the [`State`] of a [`PickList`]
/// accordingly.
pub fn update<'a, Message>(
    event: Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut Shell<'_, Message>,
    on_selected: &dyn Fn(i32) -> Message,
    selected: Option<&i32>,
    options: crate::character_change_dropdown_menu::OptionsRef<'a>,
    state: impl FnOnce() -> &'a mut State,
) -> event::Status {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            let state = state();

            let event_status = if state.is_open {
                // Event wasn' processed by overlay, so cursor was clicked either outside it's
                // bounds or on the drop-down, either way we close the overlay.
                state.is_open = state.style_menu.is_some();

                event::Status::Captured
            } else if layout.bounds().contains(cursor_position) {
                state.is_open = true;
                state.hovered_option = options
                    .iter()
                    .position(|option| Some(option.1[0].2) == selected.copied());

                event::Status::Captured
            } else {
                event::Status::Ignored
            };

            if let Some(last_selection) = state.last_selection.take() {
                shell.publish((on_selected)(last_selection));

                state.is_open = false;

                event::Status::Captured
            } else {
                event_status
            }
        }
        Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { y, .. },
        }) => {
            let state = state();

            if state.keyboard_modifiers.command()
                && layout.bounds().contains(cursor_position)
                && !state.is_open
            {
                fn find_next<'a>(
                    selected: &'a i32,
                    mut options: impl Iterator<Item = &'a i32>,
                ) -> Option<&'a i32> {
                    let _ = options.find(|&option| option == selected);

                    options.next()
                }

                let next_option = if y < 0.0 {
                    if let Some(selected) = selected {
                        find_next(selected, options.iter().map(|x| &x.1[0].2))
                    } else {
                        options.first().map(|x| &x.1[0].2)
                    }
                } else if y > 0.0 {
                    if let Some(selected) = selected {
                        find_next(selected, options.iter().map(|x| &x.1[0].2).rev())
                    } else {
                        options.last().map(|x| &x.1[0].2)
                    }
                } else {
                    None
                };

                if let Some(next_option) = next_option {
                    shell.publish((on_selected)(*next_option));
                }

                event::Status::Captured
            } else {
                event::Status::Ignored
            }
        }
        Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
            let state = state();

            state.keyboard_modifiers = modifiers;

            event::Status::Ignored
        }
        _ => event::Status::Ignored,
    }
}

/// Returns the current [`mouse::Interaction`] of a [`PickList`].
pub fn mouse_interaction(layout: Layout<'_>, cursor_position: Point) -> mouse::Interaction {
    let bounds = layout.bounds();
    let is_mouse_over = bounds.contains(cursor_position);

    if is_mouse_over {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    }
}

/// Returns the current overlay of a [`PickList`].
pub fn overlay<'a, Message, Renderer>(
    layout: Layout<'_>,
    state: &'a mut State,
    padding: Padding,
    text_size: Option<u16>,
    font: Renderer::Font,
    options: crate::character_change_dropdown_menu::OptionsRef<'a>,
    style: <Renderer::Theme as StyleSheet>::Style,
    icon_size: Size,
) -> Option<overlay::Element<'a, Message, Renderer>>
where
    Message: 'a,
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle> + 'a,
    Renderer::Theme: StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
    <Renderer::Theme as menu::StyleSheet>::Style: From<<Renderer::Theme as StyleSheet>::Style>,
{
    if state.is_open {
        let bounds = layout.bounds();

        let mut menu = Menu::new(
            &mut state.menu,
            options,
            &mut state.hovered_option,
            &mut state.hovered_notch,
            &mut state.last_selection,
            &mut state.style_menu,
            &mut state.style_menu_hovered_option,
            icon_size,
        )
        .width(300)
        .padding(padding)
        .font(font)
        .style(style);

        if let Some(text_size) = text_size {
            menu = menu.text_size(text_size);
        }

        Some(menu.overlay(layout.position(), bounds.height))
    } else {
        None
    }
}

/// Draws a [`PickList`].
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    theme: &Renderer::Theme,
    layout: Layout<'_>,
    cursor_position: Point,

    placeholder: Option<iced_native::image::Handle>,

    style: &<Renderer::Theme as StyleSheet>::Style,
) where
    Renderer: text::Renderer + image::Renderer<Handle = iced_native::image::Handle>,
    Renderer::Theme: StyleSheet,
{
    let bounds = layout.bounds();
    let is_mouse_over = bounds.contains(cursor_position);

    let style = if is_mouse_over {
        theme.hovered(style)
    } else {
        theme.active(style)
    };

    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border_color: style.border_color,
            border_width: style.border_width,
            border_radius: style.border_radius.into(),
        },
        style.background,
    );

    if let Some(handle) = placeholder {
        renderer.draw(handle, layout.bounds());
    }
}
