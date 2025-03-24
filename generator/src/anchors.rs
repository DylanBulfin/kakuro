use crate::grid_gen::{Anchor, Component};

pub const ANCHORS_8X8_1: [Anchor; 4] = [
    Anchor::new(0, 0, Component::new(2, 2)),
    Anchor::new(6, 0, Component::new(2, 2)),
    Anchor::new(0, 6, Component::new(2, 2)),
    Anchor::new(6, 6, Component::new(2, 2)),
];
//pub const ANCHORS_8X8: [Anchor; 4] = [
//    Anchor::new(0, 0, Component::new(2, 3)),
//    Anchor::new(5, 0, Component::new(3, 2)),
//    Anchor::new(0, 6, Component::new(3, 2)),
//    Anchor::new(6, 5, Component::new(2, 3)),
//];
pub const ANCHORS_8X8_2: [Anchor; 9] = [
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

pub fn generate_anchors(
    (grid_w, grid_h): (u8, u8),
    (comp_w, comp_h): (u8, u8),
    (count_v, count_h): (u8, u8),
) -> Vec<Anchor> {
    if comp_w * count_h + (count_h - 1) > grid_w || comp_h * count_v + (count_v - 1) > grid_h {
        panic!("Invalid generate_anchors call");
    }

    // Distance between each component
    let hdiff = (grid_w - (comp_w * count_h)) / (count_h - 1);
    let vdiff = (grid_h - (comp_h * count_v)) / (count_v - 1);

    let mut xs: Vec<_> = (0..count_h).map(|i| (hdiff + comp_w) * i).collect();
    let mut ys: Vec<_> = (0..count_v).map(|i| (vdiff + comp_h) * i).collect();

    let xl = xs.len();
    let yl = ys.len();

    if xs[xl - 1] + comp_h != grid_h {
        xs[xl - 1] = grid_h - comp_h;
    }
    if ys[yl - 1] + comp_w != grid_w {
        ys[yl - 1] = grid_w - comp_w;
    }

    let mut res: Vec<Anchor> = Vec::new();

    for y in ys {
        for &x in xs.iter() {
            res.push(Anchor::new(x, y, Component::new(comp_w, comp_h)));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let anchors_8x8_1 = generate_anchors((8, 8), (2, 2), (2, 2));
        let anchors_8x8_2 = generate_anchors((8, 8), (2, 2), (3, 3));
        let anchors_20x20 = generate_anchors((20, 20), (5, 5), (3, 3));

        assert_eq!(anchors_8x8_1, ANCHORS_8X8_1);
        assert_eq!(anchors_8x8_2, ANCHORS_8X8_2);
        assert_eq!(anchors_20x20, ANCHORS_20X20);
    }
}
