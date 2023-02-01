//! Build and show dropdown menus.

use iced::{
    alignment, event, mouse, overlay::menu::StyleSheet, touch, widget::Scrollable, Color, Element,
    Event, Length, Padding, Point, Rectangle, Size, Vector,
};
use iced_native::{
    image, layout, overlay,
    renderer::{self, BorderRadius},
    text::{self, Text},
    widget::{container, scrollable, Container, Tree},
    Clipboard, Layout, Shell, Widget, IME,
};

/// A list of selectable options.
#[allow(missing_debug_implementations)]
pub struct Menu<'a, Renderer>
where
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle>,
    Renderer::Theme: StyleSheet,
{
    state: &'a mut State,
    // name , styles(icon,stylename,styleid).
    options: &'a [(String, Vec<(image::Handle, String, i32)>)],
    hovered_option: &'a mut Option<usize>,
    hovered_notch: &'a mut Option<usize>,
    last_selection: &'a mut Option<i32>,
    style_menu: &'a mut Option<usize>,
    style_menu_hovered_option: &'a mut Option<usize>,
    width: u16,
    padding: Padding,
    text_size: Option<u16>,
    font: Renderer::Font,
    style: <Renderer::Theme as StyleSheet>::Style,
    icon_size: Size,
}

impl<'a, Renderer> Menu<'a, Renderer>
where
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle> + 'a,
    Renderer::Theme: StyleSheet + container::StyleSheet + scrollable::StyleSheet,
{
    /// Creates a new [`Menu`] with the given [`State`], a list of options, and
    /// the message to produced when an option is selected.
    pub fn new(
        state: &'a mut State,
        options: &'a [(String, Vec<(image::Handle, String, i32)>)],
        hovered_option: &'a mut Option<usize>,
        hovered_notch: &'a mut Option<usize>,
        last_selection: &'a mut Option<i32>,
        style_menu: &'a mut Option<usize>,
        style_menu_hovered_option: &'a mut Option<usize>,
        icon_size: Size,
    ) -> Self {
        Menu {
            state,
            options,
            hovered_option,
            last_selection,
            width: 0,
            padding: Padding::ZERO,
            text_size: None,
            font: Default::default(),
            style: Default::default(),
            style_menu,
            icon_size,
            style_menu_hovered_option,
            hovered_notch,
        }
    }

    /// Sets the width of the [`Menu`].
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Sets the [`Padding`] of the [`Menu`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`Menu`].
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the font of the [`Menu`].
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`Menu`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Turns the [`Menu`] into an overlay [`Element`] at the given target
    /// position.
    ///
    /// The `target_height` will be used to display the menu either on top
    /// of the target or under it, depending on the screen position and the
    /// dimensions of the [`Menu`].
    pub fn overlay<Message: 'a>(
        self,
        position: Point,
        target_height: f32,
    ) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(Overlay::new(self, target_height)))
    }
}

/// The local state of a [`Menu`].
#[derive(Debug)]
pub struct State {
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

struct Overlay<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: StyleSheet + container::StyleSheet,
{
    state: &'a mut Tree,

    container: Container<'a, Message, Renderer>,
    width: u16,
    target_height: f32,
}

impl<'a, Message, Renderer> Overlay<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a,
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle>,
    Renderer::Theme: StyleSheet + container::StyleSheet + scrollable::StyleSheet,
{
    pub fn new(menu: Menu<'a, Renderer>, target_height: f32) -> Self {
        let Menu {
            state,
            options,
            hovered_option,
            last_selection,
            width,
            padding,
            font,
            text_size,
            style,
            style_menu,
            icon_size,
            style_menu_hovered_option,
            hovered_notch,
        } = menu;

        let container = Container::new(Scrollable::new(List {
            options,
            hovered_option,
            last_selection,
            font,
            text_size,
            padding,
            style,
            style_menu,
            icon_size,
            style_menu_hovered_option,
            hovered_notch,
        }));

        state.tree.diff(&container as &dyn Widget<_, _>);

        Self {
            state: &mut state.tree,
            container,
            width,
            target_height,
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for Overlay<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet + container::StyleSheet,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        let space_below = bounds.height - (position.y + self.target_height);
        let space_above = position.y;

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                bounds.width - position.x,
                if space_below > space_above {
                    space_below
                } else {
                    space_above
                },
            ),
        )
        .width(Length::Units(self.width));

        let mut node = self.container.layout(renderer, &limits);

        node.move_to(if space_below > space_above {
            position + Vector::new(0.0, self.target_height)
        } else {
            position - Vector::new(0.0, node.size().height)
        });

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        _ime: &dyn IME,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.container.on_event(
            self.state,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            _ime,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.container
            .mouse_interaction(self.state, layout, cursor_position, viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();
        self.container.draw(
            self.state,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            &bounds,
        );
    }
}

struct List<'a, Renderer>
where
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle>,
    Renderer::Theme: StyleSheet,
{
    options: &'a [(String, Vec<(image::Handle, String, i32)>)],
    hovered_option: &'a mut Option<usize>,
    hovered_notch: &'a mut Option<usize>,
    last_selection: &'a mut Option<i32>,
    style_menu: &'a mut Option<usize>,
    style_menu_hovered_option: &'a mut Option<usize>,
    padding: Padding,
    text_size: Option<u16>,
    font: Renderer::Font,
    style: <Renderer::Theme as StyleSheet>::Style,
    icon_size: Size,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for List<'a, Renderer>
where
    Renderer: text::Renderer + image::Renderer<Handle = image::Handle>,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        use std::f32;

        let limits = limits.width(Length::Fill).height(Length::Shrink);
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let max_row_count = self
            .options
            .iter()
            .enumerate()
            .map(|(from_top, (_, styles))| from_top + styles.len())
            .reduce(usize::max)
            .unwrap();
        let size = {
            let intrinsic = Size::new(
                0.0,
                f32::from(text_size + self.padding.vertical()) * max_row_count as f32,
            );

            limits.resolve(intrinsic)
        };

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _ime: &dyn IME,
        _shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = layout.bounds();
                let text_size = self.text_size.unwrap_or(renderer.default_size());
                let max_text_width = self
                    .options
                    .iter()
                    .map(|(name, _)| renderer.measure_width(name, text_size, self.font.clone()))
                    .reduce(f32::max)
                    .unwrap();
                let element_height = f32::from(text_size + self.padding.vertical());

                //we need to detect each element.

                for (from_top, has_notch) in self
                    .options
                    .iter()
                    .enumerate()
                    .map(|(index, (_, submenu))| (index, submenu.len() > 1))
                {
                    let bounds = Rectangle {
                        width: max_text_width
                            + self.icon_size.width
                            + self.padding.left as f32
                            + if has_notch { 0.0 } else { text_size as f32 },
                        height: element_height,
                        x: bounds.x,
                        y: bounds.y + (element_height * from_top as f32),
                    };
                    if bounds.contains(cursor_position) {
                        if let Some(option) = self.options.get(from_top) {
                            *self.last_selection = Some(option.1[0].2);
                            *self.style_menu = None;
                            *self.style_menu_hovered_option = None;
                        }
                    } else {
                        let notch_bound = Rectangle {
                            width: text_size as f32,
                            height: element_height,
                            y: bounds.y,
                            x: bounds.x
                                + max_text_width
                                + self.icon_size.width
                                + self.padding.left as f32,
                        };
                        if notch_bound.contains(cursor_position) {
                            *self.style_menu = Some(from_top);
                            *self.style_menu_hovered_option = None;
                            *self.hovered_option = None;
                        }
                    }
                }

                if let Some(from_top) = *self.style_menu {
                    // build region.
                    if let Some(((_, style_menu), hovered_option)) = self
                        .options
                        .get(from_top)
                        .zip(*self.style_menu_hovered_option)
                    {
                        *self.last_selection = Some(style_menu[hovered_option].2);
                        *self.style_menu = None;
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();
                let max_text_width = self
                    .options
                    .iter()
                    .map(|(name, _)| {
                        renderer.measure_width(
                            name,
                            self.text_size.unwrap_or(renderer.default_size()),
                            self.font.clone(),
                        )
                    })
                    .reduce(f32::max)
                    .unwrap();

                let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
                let bounds = Rectangle {
                    width: max_text_width + self.icon_size.width + self.padding.left as f32,
                    ..bounds
                };
                let element_height = f32::from(text_size + self.padding.vertical());

                for (from_top, has_notch) in self
                    .options
                    .iter()
                    .enumerate()
                    .map(|(index, (_, submenu))| (index, submenu.len() > 1))
                {
                    let bounds = Rectangle {
                        width: bounds.width + if has_notch { 0.0 } else { text_size as f32 },
                        height: element_height,
                        x: bounds.x,
                        y: bounds.y + (element_height * from_top as f32),
                    };
                    if bounds.contains(cursor_position) {
                        *self.hovered_option = Some(from_top);
                        *self.hovered_notch = None;
                    } else {
                        let notch_bound = Rectangle {
                            width: text_size as f32,
                            height: element_height,
                            y: bounds.y,
                            x: bounds.x + bounds.width,
                        };
                        if notch_bound.contains(cursor_position) {
                            *self.hovered_notch = Some(from_top);
                            *self.hovered_option = None;
                        }
                    }
                }

                if let Some(from_top) = *self.style_menu {
                    // build region.

                    if let Some((name, style_menu)) = self.options.get(from_top) {
                        let entry_height = f32::from(text_size + self.padding.vertical());
                        let style_menu_text_width = style_menu
                            .iter()
                            .map(|(_, style_name, _)| {
                                renderer.measure_width(
                                    &format!("{name}({style_name})"),
                                    text_size,
                                    self.font.clone(),
                                )
                            })
                            .reduce(f32::max)
                            .unwrap();

                        let bounds = Rectangle {
                            x: bounds.x + bounds.width + text_size as f32,
                            y: bounds.y + entry_height * from_top as f32,
                            width: self.icon_size.width
                                + self.padding.horizontal() as f32
                                + style_menu_text_width,
                            height: entry_height * style_menu.len() as f32,
                        };
                        if bounds.contains(cursor_position) {
                            *self.style_menu_hovered_option =
                                Some(((cursor_position.y - bounds.y) / entry_height) as usize);
                        }
                    }
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();
                let text_size = self.text_size.unwrap_or(renderer.default_size());
                let max_text_width = self
                    .options
                    .iter()
                    .map(|(name, _)| renderer.measure_width(name, text_size, self.font.clone()))
                    .reduce(f32::max)
                    .unwrap();
                let element_height = f32::from(text_size + self.padding.vertical());

                //we need to detect each element.

                for (from_top, has_notch) in self
                    .options
                    .iter()
                    .enumerate()
                    .map(|(index, (_, submenu))| (index, submenu.len() > 1))
                {
                    let bounds = Rectangle {
                        width: max_text_width
                            + self.icon_size.width
                            + self.padding.left as f32
                            + if has_notch { 0.0 } else { text_size as f32 },
                        height: element_height,
                        x: bounds.x,
                        y: bounds.y + (element_height * from_top as f32),
                    };
                    if bounds.contains(cursor_position) {
                        if let Some(option) = self.options.get(from_top) {
                            *self.last_selection = Some(option.1[0].2);
                            *self.style_menu = None;
                            *self.style_menu_hovered_option = None;
                        }
                        *self.hovered_option = Some(
                            ((cursor_position.y - bounds.y)
                                / f32::from(text_size + self.padding.vertical()))
                                as usize,
                        );
                    } else {
                        let notch_bound = Rectangle {
                            width: text_size as f32,
                            height: element_height,
                            y: bounds.y,
                            x: bounds.x
                                + max_text_width
                                + self.icon_size.width
                                + self.padding.left as f32,
                        };
                        if notch_bound.contains(cursor_position) {
                            *self.style_menu = Some(from_top);
                            *self.style_menu_hovered_option = None;
                            *self.hovered_option = None;
                        }
                    }
                }

                if let Some(from_top) = *self.style_menu {
                    // build region.
                    if let Some((_, style_menu)) = self.options.get(from_top) {
                        if let Some(hovered_option) = *self.style_menu_hovered_option {
                            *self.last_selection = Some(style_menu[hovered_option].2);
                            *self.style_menu = None;
                        } 
                    } 
                }
                if let Some(from_top) = self.style_menu {
                    // build region.

                    if let Some((name, style_menu)) = self.options.get(*from_top) {
                        let entry_height = f32::from(text_size + self.padding.vertical());
                        let style_menu_text_width = style_menu
                            .iter()
                            .map(|(_, style_name, _)| {
                                renderer.measure_width(
                                    &format!("{name}({style_name})"),
                                    text_size,
                                    self.font.clone(),
                                )
                            })
                            .reduce(f32::max)
                            .unwrap();

                        let bounds = Rectangle {
                            x: bounds.x
                                + max_text_width
                                + self.icon_size.width
                                + (text_size + self.padding.left) as f32,
                            y: bounds.y + entry_height * *from_top as f32,
                            width: self.icon_size.width + style_menu_text_width,
                            height: entry_height * style_menu.len() as f32,
                        };
                        if bounds.contains(cursor_position) {
                            *self.style_menu_hovered_option =
                                Some(((cursor_position.y - bounds.y) / entry_height) as usize);
                        }
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = layout.bounds().contains(cursor_position);

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let appearance = theme.appearance(&self.style);
        let list_bounds = layout.bounds();

        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let option_height = (text_size + self.padding.vertical()) as usize;

        let offset = viewport.y - list_bounds.y;
        let start = (offset / option_height as f32) as usize;
        let end = ((offset + viewport.height) / option_height as f32).ceil() as usize;

        let visible_options = &self.options[start..end.min(self.options.len())];
        let max_text_width = visible_options
            .iter()
            .map(|(name, _)| renderer.measure_width(name, text_size, self.font.clone()))
            .reduce(f32::max)
            .unwrap();
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    width: max_text_width + self.icon_size.width + text_size as f32 - 1.0,
                    height: (self.options.len() * option_height) as f32,
                    ..list_bounds
                },
                border_color: appearance.border_color,
                border_width: appearance.border_width,
                border_radius: appearance.border_radius.into(),
            },
            appearance.background,
        );
        for (i, option) in visible_options.iter().enumerate() {
            let i = start + i;
            let is_selected = *self.hovered_option == Some(i);

            let highlight_bounds = Rectangle {
                x: list_bounds.x,
                y: list_bounds.y + (option_height * i) as f32,
                width: if option.1.len() == 1 {
                    max_text_width + self.icon_size.width + text_size as f32
                } else {
                    max_text_width + self.icon_size.width + self.padding.horizontal() as f32
                },
                height: f32::from(text_size + self.padding.vertical()),
            };

            if is_selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: highlight_bounds,
                        border_color: Color::TRANSPARENT,
                        border_width: 0.0,
                        border_radius: appearance.border_radius.into(),
                    },
                    appearance.selected_background,
                );
            }
            let icon_bounds = Rectangle::new(
                Point {
                    x: highlight_bounds.x,
                    y: highlight_bounds.y,
                },
                Size {
                    width: self.icon_size.width + self.padding.horizontal() as f32,
                    height: self.icon_size.height + self.padding.vertical() as f32,
                },
            );
            // render icon
            renderer.draw(option.1[0].0.clone(), icon_bounds);
            // render name.
            let name_bounds = Rectangle {
                x: icon_bounds.x + icon_bounds.width,
                y: highlight_bounds.center_y(),
                width: max_text_width,
                ..highlight_bounds
            };
            renderer.fill_text(Text {
                content: &option.0,
                bounds: name_bounds,
                size: f32::from(text_size),
                font: self.font.clone(),
                color: if is_selected {
                    appearance.selected_text_color
                } else {
                    appearance.text_color
                },
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Center,
            });

            if option.1.len() != 1 {
                let notch_bounds = Rectangle {
                    x: name_bounds.x + name_bounds.width + 1.0,
                    y: highlight_bounds.y,
                    width: renderer.measure_width(">", text_size, self.font.clone()),
                    ..name_bounds
                };
                let separater_bounds = Rectangle {
                    width: 1.0,
                    x: name_bounds.x + name_bounds.width,
                    y: highlight_bounds.y,
                    ..name_bounds
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: separater_bounds,
                        border_radius: BorderRadius::default(),
                        border_width: 0.0,
                        border_color: Color::BLACK,
                    },
                    Color::BLACK,
                );
                if *self.hovered_notch == Some(i) || *self.style_menu == Some(i) {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: notch_bounds,
                            border_color: Color::TRANSPARENT,
                            border_width: 0.0,
                            border_radius: appearance.border_radius.into(),
                        },
                        appearance.selected_background,
                    );
                }
                renderer.fill_text(Text {
                    content: ">",
                    bounds: Rectangle {
                        y: notch_bounds.center_y(),
                        ..notch_bounds
                    },
                    size: f32::from(text_size),
                    font: self.font.clone(),
                    color: if *self.hovered_notch == Some(i) || *self.style_menu == Some(i) {
                        appearance.selected_text_color
                    } else {
                        appearance.text_color
                    },
                    horizontal_alignment: alignment::Horizontal::Left,
                    vertical_alignment: alignment::Vertical::Center,
                });
            }
            // draw style menu to right.
            if Some(i) == *self.style_menu {
                // background paint.
                let max_width = option
                    .1
                    .iter()
                    .map(|x| {
                        renderer.measure_width(
                            &format!("{}({})", option.0, x.1),
                            text_size,
                            self.font.clone(),
                        )
                    })
                    .reduce(f32::max)
                    .unwrap_or(100.0);
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: highlight_bounds.x
                                + self.icon_size.width
                                + max_text_width
                                + text_size as f32,
                            y: highlight_bounds.y,
                            height: (option_height * option.1.len()) as f32,
                            width: self.icon_size.width + max_width + self.padding.left as f32,
                        },
                        border_radius: BorderRadius::from(0.0),
                        border_width: 1.0,
                        border_color: Color::BLACK,
                    },
                    appearance.background,
                );
                //render menu for each style.
                for (j, style) in option.1.iter().enumerate() {
                    let is_selected = *self.style_menu_hovered_option == Some(j);
                    if is_selected {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: highlight_bounds.x
                                        + self.icon_size.width
                                        + max_text_width
                                        + text_size as f32,
                                    y: highlight_bounds.y + (option_height * j) as f32,
                                    width: self.icon_size.width
                                        + max_width
                                        + self.padding.left as f32,
                                    height: f32::from(text_size + self.padding.vertical()),
                                },
                                border_color: Color::TRANSPARENT,
                                border_width: 0.0,
                                border_radius: appearance.border_radius.into(),
                            },
                            appearance.selected_background,
                        );
                    }
                    let submenu_icon_bound = Rectangle::new(
                        Point {
                            x: highlight_bounds.x
                                + self.icon_size.width
                                + max_text_width
                                + text_size as f32,
                            y: highlight_bounds.y + (option_height * j) as f32,
                        },
                        Size {
                            width: self.icon_size.width + self.padding.horizontal() as f32,
                            height: self.icon_size.height + self.padding.vertical() as f32,
                        },
                    );
                    // render icon
                    renderer.draw(style.0.clone(), submenu_icon_bound);
                    // render name.
                    renderer.fill_text(Text {
                        content: &format!("{}({})", &option.0, style.1),
                        bounds: Rectangle {
                            x: highlight_bounds.x
                                + 2.0 * self.icon_size.width
                                + max_text_width
                                + (self.padding.left + text_size) as f32,
                            y: highlight_bounds.center_y() + (option_height * j) as f32,
                            width: f32::INFINITY,
                            ..highlight_bounds
                        },
                        size: f32::from(text_size),
                        font: self.font.clone(),
                        color: if is_selected {
                            appearance.selected_text_color
                        } else {
                            appearance.text_color
                        },
                        horizontal_alignment: alignment::Horizontal::Left,
                        vertical_alignment: alignment::Vertical::Center,
                    });
                }
            }
        }
    }
}

impl<'a, Message, Renderer> From<List<'a, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + text::Renderer + image::Renderer<Handle = image::Handle>,
    Renderer::Theme: StyleSheet,
{
    fn from(list: List<'a, Renderer>) -> Self {
        Element::new(list)
    }
}
