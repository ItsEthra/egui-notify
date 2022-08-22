mod toast;
pub use toast::*;
mod anchor;
pub use anchor::*;

use egui::{Context, Vec2, vec2, LayerId, Order, Id, Color32, Rounding, FontId};

pub(crate) const TOAST_WIDTH: f32 = 180.;
pub(crate) const TOAST_HEIGHT: f32 = 34.;

pub struct Toasts {
    toasts: Vec<Toast>,
    anchor: Anchor,
    margin: Vec2,
    spacing: f32,
    vertical: bool,
    padding: Vec2,
}

impl Toasts {
    pub fn new() -> Self {
        Self {
            anchor: Anchor::BottomRight,
            margin: vec2(8., 8.),
            toasts: vec![],
            spacing: 8.,
            vertical: true,
            padding: vec2(4., 4.),
        }
    }

    pub fn add(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    pub fn show(&mut self, ctx: &Context) {
        let Self {
            anchor,
            margin,
            spacing,
            vertical,
            padding,
            toasts,
        } = self;

        let mut pos = anchor.screen_corner(ctx.input().screen_rect.max, *margin);

        let p = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("toasts")));

        for toast in toasts.iter_mut() {
            let icon_font = FontId::proportional(toast.height - padding.y * 2.);
            let icon_height = ctx.fonts().row_height(&icon_font);
            let icon_width;

            let icon_galley = if toast.level.is_info() {
                icon_width = ctx.fonts().glyph_width(&icon_font, 'ℹ');
                ctx.fonts().layout("ℹ".into(), icon_font, Color32::LIGHT_BLUE, 0.)
            } else if toast.level.is_warning() {
                icon_width = ctx.fonts().glyph_width(&icon_font, '⚠');
                ctx.fonts().layout("⚠".into(), icon_font, Color32::YELLOW, 0.)
            } else if toast.level.is_error() {
                icon_width = ctx.fonts().glyph_width(&icon_font, '！');
                ctx.fonts().layout("！".into(), icon_font, Color32::RED, 0.)
            } else {
                unreachable!()
            };
            
            let caption_galley = ctx.fonts().layout(
                toast.caption.clone(),
                FontId::proportional(16.),
                Color32::LIGHT_GRAY,
                f32::INFINITY
            );
            let caption_height = ctx.fonts().row_height(&FontId::proportional(16.));

            toast.width = toast.width.max(icon_galley.rect.width() + caption_galley.rect.width() + padding.x * 2. + icon_width);

            let rect = toast.calc_anchored_rect(pos, *anchor);

            p.rect_filled(rect, Rounding::same(4.), Color32::from_rgb(30, 30, 30));
            
            let offset = ((toast.height - padding.y * 2.) - (icon_height - padding.y * 2.)) / 2.;
            p.galley(rect.min + vec2(padding.x, offset), icon_galley);
            
            let offset = ((toast.height - padding.y * 2.) - (caption_height - padding.y * 2.)) / 2.;
            p.galley(rect.min + vec2(padding.x + icon_width + 4., offset), caption_galley);

            toast.adjust_next_pos(
                &mut pos,
                *anchor,
                *vertical,
                *spacing
            );
        }
    }
}

impl Default for Toasts {
    fn default() -> Self {
        Self::new()
    }
}