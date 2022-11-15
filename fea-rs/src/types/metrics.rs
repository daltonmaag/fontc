use write_fonts::tables::gpos::AnchorTable as RawAnchor;

#[derive(Clone, Copy, Debug)]
pub enum Anchor {
    Coord { x: i16, y: i16 },
    Contour { x: i16, y: i16, point: u16 },
    //Device { x: i32, y: i32, x_dev: (), y_dev: (), },
    Null,
    //Named(SmolStr),
}

impl Anchor {
    pub fn to_raw(self) -> Option<RawAnchor> {
        match self {
            Anchor::Coord { x, y } => Some(RawAnchor::format_1(x, y)),
            Anchor::Contour { x, y, point } => Some(RawAnchor::format_2(x, y, point)),
            Anchor::Null => None,
        }
    }
}
