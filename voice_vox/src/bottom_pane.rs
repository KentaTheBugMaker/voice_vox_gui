use crate::commands::BottomPaneCommand;

use eframe::{
    egui::{
        Align, Align2, Context, FontId, Layout, NumExt, Response, SelectableLabel, Sense,
        TextStyle, Ui, Vec2, Widget,
    },
    epaint::{self, pos2, vec2, Color32, Rect, RectShape, Rounding, Stroke},
};
use std::ops::RangeInclusive;
use voice_vox_api::api_schema::AccentPhraseInProject;

/// アクセント位置とアクセント句の変化で新しくリクエストを送る必要がある.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Displaying {
    Accent,
    Intonation,
    Length,
}

pub fn create_bottom_pane(
    current_displaying: &mut Displaying,
    should_play: &mut Option<bool>,
    ui: &mut Ui,
    edit_targets: &[AccentPhraseInProject],
) -> Option<BottomPaneCommand> {
    let mut rt = None;
    ui.horizontal(|ui| {
        use Displaying::*;

        let radius = 32.0;
        const BUTTONS: [(Displaying, &str); 3] = [
            (Accent, "アクセント"),
            (Intonation, "イントネーション"),
            (Length, "長さ"),
        ];

        let Vec2 {
            x: mut width,
            y: mut height,
        } = ui
            .painter()
            .layout(
                BUTTONS[1].1.to_owned(),
                FontId::default(),
                Default::default(),
                ui.available_width(),
            )
            .rect
            .size();

        width += 2.0 * ui.spacing().button_padding.x;
        height += 2.0 * ui.spacing().button_padding.y;

        let size = vec2(width, height * 3.0 + radius * 2.0);

        ui.add_sized(size, |ui: &mut Ui| {
            ui.vertical_centered(|ui| {
                for button in BUTTONS {
                    if ui
                        .add_sized(
                            vec2(width, height),
                            SelectableLabel::new(*current_displaying == button.0, button.1),
                        )
                        .clicked()
                    {
                        *current_displaying = button.0;
                    }
                }

                let (response, painter) =
                    ui.allocate_painter(vec2(radius * 2.0, radius * 2.0), Sense::click());
                let center = response.rect.center();
                let box_rect = response.rect.shrink(radius * (3.0 / 4.0));
                painter.circle_filled(center, radius, Color32::DARK_GREEN);

                if false {
                    let rounding = Rounding::none();
                    painter.rect(box_rect, rounding, Color32::BLACK, Stroke::none());
                    if response.clicked() {
                        should_play.replace(false);
                    }
                } else {
                    use eframe::egui::epaint::PathShape;
                    use eframe::egui::Shape;
                    let triangle_width = radius / 2.0;
                    let half_height = triangle_width / 3.0_f32.sqrt();
                    let top_left = center - vec2(triangle_width / 2.0, half_height);

                    let positions = vec![
                        top_left,
                        top_left + vec2(0.0, half_height * 2.0),
                        center + vec2(triangle_width / 2.0, 0.0),
                    ];
                    let points =
                        PathShape::convex_polygon(positions, Color32::BLACK, Stroke::none());
                    let shape = Shape::Path(points);
                    painter.add(shape);
                    if response.clicked() {
                        should_play.replace(true);
                    }
                }
            })
            .response
        });

        ui.separator();
        let alloc_height = ui.available_height() * 1.2;

        let scroll = eframe::egui::containers::ScrollArea::horizontal()
            .max_height(alloc_height)
            .auto_shrink([false, false]);
        scroll.show(ui, |ui| {
            ui.set_height(alloc_height / 1.2);
            ui.vertical(|ui| {
                let mut space = ui.spacing().item_spacing;
                space.y = ui.available_height() / 1.2;
                space.x *= 6.0;
                match current_displaying {
                    Displaying::Accent => {
                        let accent_phrase_len = edit_targets.len();
                        if !edit_targets.is_empty() {
                            ui.horizontal(|ui| {
                                for (ap, edit_target) in edit_targets.iter().enumerate() {
                                    let mut accent = edit_target.accent;
                                    let mora_len = edit_target.moras.len();
                                    let width = mora_len as f32 * space.x;
                                    ui.set_height(space.y);
                                    ui.vertical(|ui| {
                                        let slider = eframe::egui::Slider::new(
                                            &mut accent,
                                            1..=mora_len as i32,
                                        )
                                        .integer()
                                        .show_value(false);
                                        ui.style_mut().spacing.slider_width = width;
                                        let thickness = ui
                                            .text_style_height(&TextStyle::Body)
                                            .at_least(ui.spacing().interact_size.y);
                                        let radius = thickness / 2.5;
                                        let res = ui.add(slider);
                                        if (res.clicked() | res.drag_released())
                                            & (accent != edit_target.accent)
                                        {
                                            //emit signal.
                                            rt = Some(BottomPaneCommand::AccentPhrase {
                                                accent_phrase: ap,
                                                new_accent: accent as usize,
                                                prev_accent: edit_target.accent as usize,
                                            });
                                        }
                                        let h = ui.available_height();
                                        let w = res.rect.width();
                                        let (r, painter) = ui.allocate_painter(
                                            vec2(w, h),
                                            Sense::focusable_noninteractive(),
                                        );
                                        let rect = r.rect;

                                        let left = rect.left();
                                        let top = rect.top();
                                        let bottom = rect.bottom();

                                        let text_height = thickness;
                                        //
                                        let mut graph_pos = bottom - text_height;

                                        let mut line_points = vec![];
                                        let width_per_mora = (w - radius * 2.0)
                                            / ((edit_target.moras.len() - 1) as f32);
                                        for (idx, mora) in edit_target.moras.iter().enumerate() {
                                            let x = width_per_mora * idx as f32;
                                            if (idx + 1) == edit_target.accent as usize {
                                                painter.vline(
                                                    left + x + radius,
                                                    top..=bottom - text_height,
                                                    Stroke::new(2.0, Color32::LIGHT_GREEN),
                                                );
                                            } else {
                                                let dash_len = height / 10.0;
                                                painter.add(Shape::dashed_line(
                                                    &[
                                                        pos2(left + x + radius, top),
                                                        pos2(
                                                            left + x + radius,
                                                            bottom - text_height,
                                                        ),
                                                    ],
                                                    Stroke::new(1.0, Color32::LIGHT_GREEN),
                                                    dash_len,
                                                    dash_len,
                                                ));
                                            };

                                            painter.text(
                                                pos2(left + x, bottom - text_height),
                                                Align2::LEFT_TOP,
                                                &mora.text,
                                                FontId::default(),
                                                Color32::LIGHT_GRAY,
                                            );
                                            if idx + 1 == accent as usize {
                                                graph_pos = top;
                                            } else if idx + 1 > accent as usize {
                                                graph_pos = bottom - text_height;
                                            } else if (idx + 1 < accent as usize) & (idx + 1 != 1) {
                                                graph_pos = top;
                                            }
                                            line_points.push(pos2(left + x + radius, graph_pos));
                                        }
                                        use epaint::Shape;
                                        let shape = Shape::line(
                                            line_points,
                                            Stroke::new(2.0, Color32::LIGHT_GRAY),
                                        );

                                        painter.add(shape);
                                    });

                                    if ap < accent_phrase_len - 1 {
                                        let button = eframe::egui::Button::new("");
                                        if ui.add_sized(space, button).clicked() {
                                            rt = Some(BottomPaneCommand::Concat {
                                                accent_phrase: ap,
                                                length: mora_len,
                                            });
                                        }
                                    }
                                }
                            });
                        }
                    }
                    Displaying::Intonation => {
                        let accent_phrase_len = edit_targets.len();
                        if !edit_targets.is_empty() {
                            ui.horizontal(|ui| {
                                for (ap, edit_target) in edit_targets.iter().enumerate() {
                                    let mora_len = edit_target.moras.len();
                                    for (index, mora) in edit_target.moras.iter().enumerate() {
                                        let mut pitch = mora.pitch;
                                        let slider =
                                            eframe::egui::Slider::new(&mut pitch, 3.0..=6.5)
                                                .vertical()
                                                .text(&mora.text)
                                                .show_value(false);
                                        let res = ui.add(slider);

                                        if (res.clicked() | res.drag_released())
                                            & ((pitch - mora.pitch).abs() > f32::EPSILON)
                                        {
                                            //emit signal.
                                            rt = Some(BottomPaneCommand::Pitch {
                                                accent_phrase: ap,
                                                mora: index,
                                                pitch_diff: pitch - mora.pitch,
                                            });
                                        }
                                    }
                                    if ap < accent_phrase_len - 1 {
                                        let button = eframe::egui::Button::new("");
                                        if ui.add_sized(space, button).clicked() {
                                            rt = Some(BottomPaneCommand::Concat {
                                                accent_phrase: ap,
                                                length: mora_len,
                                            });
                                        }
                                    }
                                }
                            });
                        }
                    }
                    Displaying::Length => {
                        let accent_phrase_len = edit_targets.len();
                        if !edit_targets.is_empty() {
                            println!("space.y {}", space.y);
                            ui.horizontal(|ui| {
                                for (ap, edit_target) in edit_targets.iter().enumerate() {
                                    let mora_len = edit_target.moras.len();
                                    for (index, mora) in edit_target.moras.iter().enumerate() {
                                        if let Some(prev_consonant) = mora.consonantLength {
                                            let mut consonant = prev_consonant;
                                            let mut vowel = mora.vowelLength;
                                            let slider = TwoNotchSlider {
                                                a: &mut consonant,
                                                b: &mut vowel,
                                                range: 0.0..=0.30,
                                                text: mora.text.clone(),
                                            };
                                            let res = ui.add(slider);

                                            let vowel_diff = if (res.clicked()
                                                | res.drag_released())
                                                & ((vowel - mora.vowelLength).abs() > f32::EPSILON)
                                            {
                                                log::debug!("vowel {}", vowel);
                                                Some(vowel - mora.vowelLength)
                                            } else {
                                                None
                                            };
                                            let consonant_diff = if (res.clicked()
                                                | res.drag_released())
                                                & ((consonant - prev_consonant).abs()
                                                    > f32::EPSILON)
                                            {
                                                log::debug!("consonant {}", consonant);
                                                Some(consonant - prev_consonant)
                                            } else {
                                                None
                                            };
                                            if res.clicked() | res.drag_released() {
                                                //emit signal.
                                                rt = Some(BottomPaneCommand::VowelAndConsonant {
                                                    accent_phrase: ap,
                                                    mora: index,
                                                    vowel_diff,
                                                    consonant_diff,
                                                });
                                            }
                                        } else {
                                            let mut vowel = mora.vowelLength;
                                            let slider =
                                                eframe::egui::Slider::new(&mut vowel, 0.0..=0.30)
                                                    .vertical()
                                                    .text(&mora.text)
                                                    .show_value(false);
                                            let res = ui.add(slider);

                                            if (res.clicked() | res.drag_released())
                                                & ((vowel - mora.vowelLength).abs() > f32::EPSILON)
                                            {
                                                //emit signal.
                                                rt = Some(BottomPaneCommand::VowelAndConsonant {
                                                    accent_phrase: ap,
                                                    mora: index,
                                                    vowel_diff: Some(vowel - mora.vowelLength),
                                                    consonant_diff: None,
                                                });
                                            }
                                        };
                                    }
                                    if ap < accent_phrase_len - 1 {
                                        let button = eframe::egui::Button::new("");
                                        if ui.add_sized(space, button).clicked() {
                                            rt = Some(BottomPaneCommand::Concat {
                                                accent_phrase: ap,
                                                length: mora_len,
                                            });
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            });
        });
    });
    rt
}

pub struct TwoNotchSlider<'a> {
    pub a: &'a mut f32,
    pub b: &'a mut f32,
    pub range: RangeInclusive<f32>,
    pub text: String,
}

impl<'a> TwoNotchSlider<'a> {
    /// just slider space no text.
    fn allocate_slider_space(&self, ui: &mut Ui) -> (Response, Rect, Response) {
        let slider_width = ui.spacing().slider_width;
        let rail_width = ui
            .text_style_height(&TextStyle::Body)
            .at_least(ui.spacing().interact_size.y)
            / 3.0;
        let desired_size = vec2(rail_width, slider_width);
        ui.with_layout(eframe::egui::Layout::left_to_right(Align::Min), |ui| {
            let left = ui.allocate_response(desired_size, Sense::click_and_drag());
            let center_rail_rect = left.rect.translate(vec2(rail_width, 0.0));
            ui.painter().add(RectShape {
                rect: center_rail_rect,
                rounding: ui.visuals().widgets.inactive.rounding,
                fill: Color32::LIGHT_GRAY,
                stroke: Stroke::default(),
            });
            let right = ui.allocate_response(desired_size, Sense::click_and_drag());
            (left, center_rail_rect, right)
        })
        .inner
    }
    fn slider_ui(self, ui: &mut Ui) -> Response {
        let response = self.allocate_slider_space(ui);
        let res_left = response.0;
        let res_right = response.2;

        let notch = self.notch(&response.1);
        ui.painter().add(epaint::RectShape {
            rect: notch.0,
            rounding: Rounding::none(),
            fill: Color32::BLUE,
            stroke: Stroke::default(),
        });
        ui.painter().add(epaint::RectShape {
            rect: notch.1,
            rounding: Rounding::none(),
            fill: Color32::BLUE,
            stroke: Stroke::default(),
        });

        res_left.union(res_right)
    }
    // returns each notch rect ready to paint.
    fn notch(&self, rail_rect: &Rect) -> (Rect, Rect) {
        let notch_size = rail_rect.width();
        let height = rail_rect.height();
        let a = (*self.a - self.range.start()) / (self.range.end() - self.range.start());
        let b = (*self.b - self.range.start()) / (self.range.end() - self.range.start());
        let a = height - a * height;
        let b = height - b * height;

        let a_center_x = rail_rect.left() - 0.75 * notch_size;
        let b_center_y = rail_rect.right() + 0.75 * notch_size;
        let a_center = pos2(a_center_x, rail_rect.top() + a);
        let b_center = pos2(b_center_y, rail_rect.top() + b);
        let size = vec2(notch_size * 2.0, notch_size * 2.0);
        let a_rect = Rect::from_center_size(a_center, size);
        let b_rect = Rect::from_center_size(b_center, size);
        (a_rect, b_rect)
    }
}

impl<'a> Widget for TwoNotchSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.label(&self.text);
            self.slider_ui(ui)
        })
        .inner
    }
}
