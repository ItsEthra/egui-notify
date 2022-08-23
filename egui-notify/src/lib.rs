//! egui-notify
//! Simple notifications library for EGUI

#![warn(missing_docs)]

mod toast;
pub use toast::*;
mod anchor;
pub use anchor::*;

use egui::{Context, Vec2, vec2, LayerId, Order, Id, Color32, Rounding, FontId, Rect, Stroke};
#[doc(hidden)]
pub use egui::__run_test_ctx;

pub(crate) const TOAST_WIDTH: f32 = 180.;
pub(crate) const TOAST_HEIGHT: f32 = 34.;

/// Main notifications collector.
/// # Usage
/// You need to create [`Toasts`] once and call `.show(ctx)` in every frame.
/// ```
/// use egui_notify::Toasts;
/// 
/// # egui_notify::__run_test_ctx(|ctx| {
/// let mut t = Toasts::default();
/// t.info("Hello, World!", |t| t.with_duration(5.));
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
    default_time: f32,

    held: bool,
}

impl Toasts {
    /// Creates new [`Toasts`] instance.
    pub fn new() -> Self {
        Self {
            anchor: Anchor::BottomRight,
            margin: vec2(8., 8.),
            toasts: vec![],
            spacing: 8.,
            padding: vec2(4., 4.),
            held: false,
            speed: 4.,
            reverse: false,
            default_time: 5.,
        }
    }

    /// Adds new toast to the collection.
    /// By default adds toast at the end of the list, can be changed with `self.reverse`.
    pub fn add(&mut self, mut toast: Toast) {
        if toast.expires && toast.duration.is_none() {
            toast.initial_duration = Some(self.default_time);
            toast.duration = Some(self.default_time);
        }

        if self.reverse {
            self.toasts.insert(0, toast);
        } else {
            self.toasts.push(toast);
        }
    }

    /// Shortcut for adding a toast with info `level`.
    pub fn info(&mut self, caption: impl Into<String>, cb: impl FnOnce(Toast) -> Toast) {
        self.add(cb(Toast::info(caption)));
    }

    /// Shortcut for adding a toast with warning `level`.
    pub fn warning(&mut self, caption: impl Into<String>, cb: impl FnOnce(Toast) -> Toast) {
        self.add(cb(Toast::warning(caption)));
    }

    /// Shortcut for adding a toast with error `level`.
    pub fn error(&mut self, caption: impl Into<String>, cb: impl FnOnce(Toast) -> Toast) {
        self.add(cb(Toast::error(caption)));
    }

    /// Should toasts be added in reverse order?
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Where toasts should appear.
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets spacing between adjacent toasts.
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Margin or distance from screen to toasts' bounding boxes
    pub fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    /// Padding or distance from toasts' bounding boxes to inner contents.
    pub fn with_padding(mut self, padding: Vec2) -> Self {
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

        let mut pos = anchor.screen_corner(ctx.input().screen_rect.max, *margin);
        let p = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("toasts")));

        let mut remove = None;

        toasts.retain(|t| t.duration.map(|d| d > 0.).unwrap_or(true));

        if ctx.input().pointer.primary_released() {
            *held = false;
        }

        let mut update = false;

        for (i,toast) in toasts.iter_mut().enumerate() {
            if let Some(d) = toast.duration.as_mut() && toast.state.idling() {
                *d -= ctx.input().stable_dt;
                update = true;
            }

            let icon_font = FontId::proportional(toast.height - padding.y * 2.);

            let icon_galley = if toast.level.is_info() {
                ctx.fonts().layout("ℹ".into(), icon_font, Color32::LIGHT_BLUE, f32::INFINITY)
            } else if toast.level.is_warning() {
                ctx.fonts().layout("⚠".into(), icon_font, Color32::YELLOW, f32::INFINITY)
            } else if toast.level.is_error() {
                ctx.fonts().layout("！".into(), icon_font, Color32::RED, f32::INFINITY)
            } else {
                unreachable!()
            };
            let (icon_width, icon_height) = (icon_galley.rect.width(), icon_galley.rect.height());
            
            let caption_galley = ctx.fonts().layout(
                toast.caption.clone(),
                FontId::proportional(16.),
                Color32::LIGHT_GRAY,
                f32::INFINITY
            );
            let caption_height = caption_galley.rect.height();

            toast.width = toast.width.max(icon_galley.rect.width() + caption_galley.rect.width() + padding.x * 2. + icon_width + 6.);

            let anim_offset = toast.width * (1. - toast.value);
            pos.x += anim_offset * anchor.anim_side();
            let rect = toast.calc_anchored_rect(pos, *anchor);
            pos.x -= anim_offset * anchor.anim_side();
            
            let toast_hovered = ctx.input().pointer.hover_pos().map(|p| rect.contains(p)).unwrap_or(false);

            p.rect_filled(rect, Rounding::same(4.), Color32::from_rgb(30, 30, 30));
            
            let oy = ((toast.height - padding.y * 2.) - (icon_height - padding.y * 2.)) / 2.;
            p.galley(rect.min + vec2(padding.x, oy), icon_galley);
            
            let oy = ((toast.height - padding.y * 2.) - (caption_height - padding.y * 2.)) / 2.;
            p.galley(rect.min + vec2(padding.x + icon_width + 4., oy), caption_galley);

            if let Some((current, initial)) = toast.duration.zip(toast.initial_duration) {
                p.line_segment([
                    rect.min + vec2(0., toast.height),
                    rect.max - vec2((1. - (current / initial)) * toast.width, 0.)
                ], Stroke::new(2., Color32::LIGHT_GRAY));
            }

            if toast.closable {
                let cross_fid = FontId::proportional(toast.height - padding.y * 2.);
                let cross_galley = ctx.fonts().layout(
                    "❌".into(),
                    cross_fid,
                    if toast_hovered { Color32::WHITE } else { Color32::GRAY },
                    f32::INFINITY
                );
                let cross_width = cross_galley.rect.width();
                let cross_height = cross_galley.rect.height();
                let cross_rect = cross_galley.rect;
    
                let oy = ((toast.height - padding.y * 2.) - (cross_height - padding.y * 2.)) / 2.;
                let mut cross_pos = rect.min + vec2(0., oy);
                cross_pos.x = rect.max.x - cross_width - padding.x;
                p.galley(cross_pos, cross_galley);

                let screen_cross = Rect {
                    max: cross_pos + cross_rect.max.to_vec2(),
                    min: cross_pos,
                };

                if let Some(pos) = ctx.input().pointer.press_origin() && screen_cross.contains(pos) && !*held {
                    remove = Some(i);
                    *held = true;
                }
            }

            toast.adjust_next_pos(
                &mut pos,
                *anchor,
                *spacing
            );

            // Animations
            if toast.state.appearing() {
                update = true;
                toast.value += ctx.input().stable_dt * *speed;
                
                if toast.value >= 1. {
                    toast.value = 1.;
                    toast.state = ToastState::Idle;
                }
            }
        }

        if update {
            ctx.request_repaint();
        }

        if let Some(del) = remove {
            self.toasts.remove(del);
        }
    }
}

impl Default for Toasts {
    fn default() -> Self {
        Self::new()
    }
}