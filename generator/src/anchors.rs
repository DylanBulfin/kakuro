use crate::generator::{Anchor, Component};

pub const ANCHORS_8X8: [Anchor; 4] = [
    Anchor::new(0, 0, Component::new(2, 2)),
    Anchor::new(6, 0, Component::new(2, 2)),
    Anchor::new(0, 6, Component::new(2, 2)),
    Anchor::new(6, 6, Component::new(2, 2)),
];
