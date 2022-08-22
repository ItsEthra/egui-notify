use egui::{Pos2, pos2, Vec2};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Anchor {
    pub(crate) fn screen_corner(&self, sc: Pos2, margin: Vec2) -> Pos2 {
        let mut out = match self {
            Anchor::TopRight => pos2(sc.x, 0.),
            Anchor::TopLeft => pos2(0., 0.),
            Anchor::BottomRight => sc,
            Anchor::BottomLeft => pos2(0., sc.y),
        };
        self.apply_margin(&mut out, margin);
        out
    }
    
    pub(crate) fn apply_margin(&self, pos: &mut Pos2, margin: Vec2) {
        match self {
            Anchor::TopRight => {
                pos.x -= margin.x;
                pos.y += margin.y;
            },
            Anchor::TopLeft => {
                pos.x += margin.x;
                pos.y += margin.y
            },
            Anchor::BottomRight => {
                pos.x -= margin.x;
                pos.y -= margin.y;
            },
            Anchor::BottomLeft => {
                pos.x += margin.x;
                pos.y -= margin.y;
            },
        }
    }
}