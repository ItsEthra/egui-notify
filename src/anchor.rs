use egui::{pos2, Pos2, Vec2};

/// Anchor where to show toasts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Top right corner.
    TopRight,
    /// Top left corner.
    TopLeft,
    /// Top middle.
    TopMiddle,
    /// Bottom right corner.
    BottomRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom middle
    BottomMiddle,
}

impl Anchor {
    #[inline]
    pub(crate) const fn anim_side(&self) -> f32 {
        match self {
            Self::TopRight | Self::BottomRight | Self::TopMiddle | Self::BottomMiddle => 1.,
            Self::TopLeft | Self::BottomLeft => -1.,
        }
    }
}

impl Anchor {
    pub(crate) fn screen_corner(&self, sc: Pos2, margin: Vec2) -> Pos2 {
        let mut out = match self {
            Self::TopRight => pos2(sc.x, 0.),
            Self::TopMiddle => pos2(sc.x / 2.0, 0.),
            Self::TopLeft => pos2(0., 0.),
            Self::BottomRight => sc,
            Self::BottomMiddle => pos2(sc.x / 2.0, sc.y),
            Self::BottomLeft => pos2(0., sc.y),
        };
        self.apply_margin(&mut out, margin);
        out
    }

    pub(crate) fn apply_margin(&self, pos: &mut Pos2, margin: Vec2) {
        match self {
            Self::TopRight => {
                pos.x -= margin.x;
                pos.y += margin.y;
            }
            Self::TopMiddle => {
                pos.y += margin.y;
            }
            Self::TopLeft => {
                pos.x += margin.x;
                pos.y += margin.y;
            }
            Self::BottomRight => {
                pos.x -= margin.x;
                pos.y -= margin.y;
            }
            Self::BottomMiddle => {
                pos.y -= margin.y;
            }
            Self::BottomLeft => {
                pos.x += margin.x;
                pos.y -= margin.y;
            }
        }
    }
}
