use crate::generator::{Anchor, Component};

//pub const ANCHORS_8X8: [Anchor; 4] = [
//    Anchor::new(0, 0, Component::new(2, 2)),
//    Anchor::new(6, 0, Component::new(2, 2)),
//    Anchor::new(0, 6, Component::new(2, 2)),
//    Anchor::new(6, 6, Component::new(2, 2)),
//];
//pub const ANCHORS_8X8: [Anchor; 4] = [
//    Anchor::new(0, 0, Component::new(2, 3)),
//    Anchor::new(5, 0, Component::new(3, 2)),
//    Anchor::new(0, 6, Component::new(3, 2)),
//    Anchor::new(6, 5, Component::new(2, 3)),
//];
pub const ANCHORS_8X8: [Anchor; 9] = [
    Anchor::new(0, 0, Component::new(2, 2)),
    Anchor::new(3, 0, Component::new(2, 2)),
    Anchor::new(6, 0, Component::new(2, 2)),
    Anchor::new(0, 3, Component::new(2, 2)),
    Anchor::new(3, 3, Component::new(2, 2)),
    Anchor::new(6, 3, Component::new(2, 2)),
    Anchor::new(0, 6, Component::new(2, 2)),
    Anchor::new(3, 6, Component::new(2, 2)),
    Anchor::new(6, 6, Component::new(2, 2)),
];

pub const ANCHORS_20X20: [Anchor; 9] = [
    Anchor::new(0, 0, Component::new(5, 5)),
    Anchor::new(7, 0, Component::new(5, 5)),
    Anchor::new(15, 0, Component::new(5, 5)),
    Anchor::new(0, 7, Component::new(5, 5)),
    Anchor::new(7, 7, Component::new(5, 5)),
    Anchor::new(15, 7, Component::new(5, 5)),
    Anchor::new(0, 15, Component::new(5, 5)),
    Anchor::new(7, 15, Component::new(5, 5)),
    Anchor::new(15, 15, Component::new(5, 5)),
];
