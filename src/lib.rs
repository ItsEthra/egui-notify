//! egui-notify
//! Simple notifications library for EGUI

#![warn(missing_docs)]

mod toast;
pub use toast::*;
mod anchor;
pub use anchor::*;

#[doc(hidden)]
pub use egui::__run_test_ctx;
use egui::{vec2, Color32, Context, FontId, Id, LayerId, Order, Rect, Rounding, Stroke, Vec2};

pub(crate) const TOAST_WIDTH: f32 = 180.;
pub(crate) const TOAST_HEIGHT: f32 = 34.;

const ERROR_COLOR: Color32 = Color32::from_rgb(200, 90, 90);
const INFO_COLOR: Color32 = Color32::from_rgb(150, 200, 210);
const WARNING_COLOR: Color32 = Color32::from_rgb(230, 220, 140);
const SUCCESS_COLOR: Color32 = Color32::from_rgb(140, 230, 140);

/// Main notifications collector.
/// # Usage
/// You need to create [`Toasts`] once and call `.show(ctx)` in every frame.
/// ```
/// # use std::time::Duration;
/// use egui_notify::Toasts;
///
/// # egui_notify::__run_test_ctx(|ctx| {
/// let mut t = Toasts::default();
/// t.info("Hello, World!").set_duration(Some(Duration::from_secs(5))).set_closable(true);
/// // More app code
/// t.show(ctx);
/// # });
/// ```
pub struct Toasts {
    toasts: Vec<Toast>,
    anchor: Anchor,
    margin: Vec2,
    spacing: f32,
    padding: Vec2,
    reverse: bool,
    speed: f32,

    held: bool,
}

impl Toasts {
    /// Creates new [`Toasts`] instance.
    pub const fn new() -> Self {
        Self {
            anchor: Anchor::TopRight,
            margin: vec2(8., 8.),
            toasts: vec![],
            spacing: 8.,
            padding: vec2(10., 10.),
            held: false,
            speed: 4.,
            reverse: false,
        }
    }

    /// Adds new toast to the collection.
    /// By default adds toast at the end of the list, can be changed with `self.reverse`.
    pub fn add(&mut self, toast: Toast) -> &mut Toast {
        if self.reverse {
            self.toasts.insert(0, toast);
            return self.toasts.get_mut(0).unwrap();
        } else {
            self.toasts.push(toast);
            let l = self.toasts.len() - 1;
            return self.toasts.get_mut(l).unwrap();
        }
    }

    /// Dismisses the oldest toast
    pub fn dismiss_oldest_toast(&mut self) {
        if let Some(toast) = self.toasts.get_mut(0) {
            toast.dismiss();
        }
    }

    /// Dismisses the most recent toast
    pub fn dismiss_latest_toast(&mut self) {
        if let Some(toast) = self.toasts.last_mut() {
            toast.dismiss();
        }
    }

    /// Dismisses all toasts
    pub fn dismiss_all_toasts(&mut self) {
        for toast in self.toasts.iter_mut() {
            toast.dismiss();
        }
    }

    /// Shortcut for adding a toast with info `success`.
    pub fn success(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::success(caption))
    }

    /// Shortcut for adding a toast with info `level`.
    pub fn info(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::info(caption))
    }

    /// Shortcut for adding a toast with warning `level`.
    pub fn warning(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::warning(caption))
    }

    /// Shortcut for adding a toast with error `level`.
    pub fn error(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::error(caption))
    }

    /// Shortcut for adding a toast with no level.
    pub fn basic(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::basic(caption))
    }

    /// Should toasts be added in reverse order?
    pub const fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Where toasts should appear.
    pub const fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets spacing between adjacent toasts.
    pub const fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Margin or distance from screen to toasts' bounding boxes
    pub const fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    /// Padding or distance from toasts' bounding boxes to inner contents.
    pub const fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
}

impl Toasts {
    /// Displays toast queue
    pub fn show(&mut self, ctx: &Context) {
        let Self {
            anchor,
            margin,
            spacing,
            padding,
            toasts,
            held,
            speed,
            ..
        } = self;

        let mut pos = anchor.screen_corner(ctx.input(|i| i.screen_rect.max), *margin);
        let p = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("toasts")));

        let mut dismiss = None;

        // Remove disappeared toasts
        toasts.retain(|t| !t.state.disappeared());

        // Start disappearing expired toasts
        toasts.iter_mut().for_each(|t| {
            if let Some((_initial_d, current_d)) = t.duration {
                if current_d <= 0. {
                    t.state = ToastState::Disapper
                }
            }
        });

        // `held` used to prevent sticky removal
        if ctx.input(|i| i.pointer.primary_released()) {
            *held = false;
        }

        let visuals = ctx.style().visuals.widgets.noninteractive;
        let mut update = false;

        for (i, toast) in toasts.iter_mut().enumerate() {
            // Decrease duration if idling
            if let Some((_, d)) = toast.duration.as_mut() {
                if toast.state.idling() {
                    *d -= ctx.input(|i| i.stable_dt);
                    update = true;
                }
            }

            // Create toast label
            let caption_galley = ctx.fonts(|f| {
                f.layout(
                    toast.caption.clone(),
                    FontId::proportional(16.),
                    visuals.fg_stroke.color,
                    f32::INFINITY,
                )
            });

            let (caption_width, caption_height) =
                (caption_galley.rect.width(), caption_galley.rect.height());

            let line_count = toast.caption.chars().filter(|c| *c == '\n').count() + 1;
            let icon_width = caption_height / line_count as f32;

            // Create toast icon
            let icon_font = FontId::proportional(icon_width);
            let icon_galley = if matches!(toast.level, ToastLevel::Info) {
                Some(ctx.fonts(|f| f.layout("ℹ".into(), icon_font, INFO_COLOR, f32::INFINITY)))
            } else if matches!(toast.level, ToastLevel::Warning) {
                Some(ctx.fonts(|f| f.layout("⚠".into(), icon_font, WARNING_COLOR, f32::INFINITY)))
            } else if matches!(toast.level, ToastLevel::Error) {
                Some(ctx.fonts(|f| f.layout("！".into(), icon_font, ERROR_COLOR, f32::INFINITY)))
            } else if matches!(toast.level, ToastLevel::Success) {
                Some(ctx.fonts(|f| f.layout("✅".into(), icon_font, SUCCESS_COLOR, f32::INFINITY)))
            } else {
                None
            };

            let (action_width, action_height) = if let Some(icon_galley) = icon_galley.as_ref() {
                (icon_galley.rect.width(), icon_galley.rect.height())
            } else {
                (0., 0.)
            };

            // Create closing cross
            let cross_galley = if toast.closable {
                let cross_fid = FontId::proportional(icon_width);
                let cross_galley = ctx.fonts(|f| {
                    f.layout(
                        "❌".into(),
                        cross_fid,
                        visuals.fg_stroke.color,
                        f32::INFINITY,
                    )
                });
                Some(cross_galley)
            } else {
                None
            };

            let (cross_width, cross_height) = if let Some(cross_galley) = cross_galley.as_ref() {
                (cross_galley.rect.width(), cross_galley.rect.height())
            } else {
                (0., 0.)
            };

            let icon_x_padding = (0., 7.);
            let cross_x_padding = (7., 0.);

            let icon_width_padded = if icon_width == 0. {
                0.
            } else {
                icon_width + icon_x_padding.0 + icon_x_padding.1
            };
            let cross_width_padded = if cross_width == 0. {
                0.
            } else {
                cross_width + cross_x_padding.0 + cross_x_padding.1
            };

            toast.width = icon_width_padded + caption_width + cross_width_padded + (padding.x * 2.);
            toast.height = action_height.max(caption_height).max(cross_height) + padding.y * 2.;

            let anim_offset = toast.width * (1. - ease_in_cubic(toast.value));
            pos.x += anim_offset * anchor.anim_side();
            let rect = toast.calc_anchored_rect(pos, *anchor);

            // Required due to positioning of the next toast
            pos.x -= anim_offset * anchor.anim_side();

            // Draw background
            p.rect_filled(rect, Rounding::same(4.), visuals.bg_fill);

            // Paint icon
            if let Some((icon_galley, true)) =
                icon_galley.zip(Some(toast.level != ToastLevel::None))
            {
                let oy = toast.height / 2. - action_height / 2.;
                let ox = padding.x + icon_x_padding.0;
                p.galley(rect.min + vec2(ox, oy), icon_galley);
            }

            // Paint caption
            let oy = toast.height / 2. - caption_height / 2.;
            let o_from_icon = if action_width == 0. {
                0.
            } else {
                action_width + icon_x_padding.1
            };
            let o_from_cross = if cross_width == 0. {
                0.
            } else {
                cross_width + cross_x_padding.0
            };
            let ox = (toast.width / 2. - caption_width / 2.) + o_from_icon / 2. - o_from_cross / 2.;
            p.galley(rect.min + vec2(ox, oy), caption_galley);

            // Paint cross
            if let Some(cross_galley) = cross_galley {
                let cross_rect = cross_galley.rect;
                let oy = toast.height / 2. - cross_height / 2.;
                let ox = toast.width - cross_width - cross_x_padding.1 - padding.x;
                let cross_pos = rect.min + vec2(ox, oy);
                p.galley(cross_pos, cross_galley);

                let screen_cross = Rect {
                    max: cross_pos + cross_rect.max.to_vec2(),
                    min: cross_pos,
                };

                if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                    if screen_cross.contains(pos) && !*held {
                        dismiss = Some(i);
                        *held = true;
                    }
                }
            }

            // Draw duration
            if toast.show_progress_bar {
                if let Some((initial, current)) = toast.duration {
                    if !toast.state.disappearing() {
                        p.line_segment(
                            [
                                rect.min + vec2(0., toast.height),
                                rect.max - vec2((1. - (current / initial)) * toast.width, 0.),
                            ],
                            Stroke::new(4., visuals.fg_stroke.color),
                        );
                    }
                }
            }

            toast.adjust_next_pos(&mut pos, *anchor, *spacing);

            // Animations
            if toast.state.appearing() {
                update = true;
                toast.value += ctx.input(|i| i.stable_dt) * (*speed);

                if toast.value >= 1. {
                    toast.value = 1.;
                    toast.state = ToastState::Idle;
                }
            } else if toast.state.disappearing() {
                update = true;
                toast.value -= ctx.input(|i| i.stable_dt) * (*speed);

                if toast.value <= 0. {
                    toast.state = ToastState::Disappeared;
                }
            }
        }

        if update {
            ctx.request_repaint();
        }

        if let Some(i) = dismiss {
            self.toasts[i].dismiss();
        }
    }
}

impl Default for Toasts {
    fn default() -> Self {
        Self::new()
    }
}

fn ease_in_cubic(x: f32) -> f32 {
    1. - (1. - x).powi(3)
}
