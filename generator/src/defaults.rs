const DEFAULTS_2X2: [[bool; 3]; 3] = [
    [false, true, true],
    [true, false, false],
    [true, false, false],
];

struct Component {
    width: u8,
    height: u8,
    corners: [Corner; 4], // These are differentials and can be up and to the right, so they can be negative
}

impl Component {
    pub const fn new(width: u8, height: u8, corners: [Corner; 4]) -> Self {
        Self {
            width,
            height,
            corners,
        }
    }
}

struct Corner {
    pos: (u8, u8),
    openings: [(i8, i8); 2],
}

impl Corner {
    pub const fn new(x: u8, y: u8, opening1: (i8, i8), opening2: (i8, i8)) -> Self {
        Self {
            pos: (x, y),
            openings: [opening1, opening2],
        }
    }

    pub const fn create_corners(width: u8, height: u8) -> [Corner; 4] {
        let (x, y) = (width - 1, height - 1);
        [
            Corner::new(0, 0, (-1, 0), (0, -1)),
            Corner::new(x, 0, (1, 0), (0, -1)),
            Corner::new(0, y, (-1, 0), (0, 1)),
            Corner::new(x, y, (1, 0), (0, 1)),
        ]
    }
}

const COMPONENTS: [Component; 7] = [
    Component::new(2, 2, Corner::create_corners(2, 2)),
    Component::new(2, 3, Corner::create_corners(2, 3)),
    Component::new(3, 2, Corner::create_corners(3, 2)),
    Component::new(3, 3, Corner::create_corners(3, 3)),
    Component::new(3, 4, Corner::create_corners(3, 4)),
    Component::new(4, 3, Corner::create_corners(4, 3)),
    Component::new(4, 4, Corner::create_corners(4, 4)),
];

enum ConnectorDir {
    TOP_LEFT,
    TOP_RIGHT,
    BOT_LEFT,
    BOT_RIGHT,
}

enum Connector {
    Merged, // One component's corners merge with another's
    None,   // One component directly next to other
    OneCell,
    TwoCell,
    ThreeCell,
    FiveCell,
}

impl Connector {
    fn get_base_diff(&self) -> (i8, i8) {
        match self {
            Self::Merged => (0, 0),
            Self::None => (1, 0),
            Self::OneCell => (1, 1),
            Self::TwoCell => (2, 1),
            Self::ThreeCell => (2, 2),
            Self::FiveCell => (3, 3),
        }
    }

    // TODO extend this to also return an array of cells associated with each diff
    pub fn get_all_diffs(&self, dir: ConnectorDir) -> Vec<(i8, i8)> {
        let (x, y) = self.get_base_diff();
        let (mx, my) = match dir {
            ConnectorDir::TOP_LEFT => (-1, -1),
            ConnectorDir::TOP_RIGHT => (1, -1),
            ConnectorDir::BOT_LEFT => (-1, 1),
            ConnectorDir::BOT_RIGHT => (1, 1),
        };

        let pos = (x * mx, y * my);
        let rpos = (y * mx, x * my);

        if x == y {
            // Symmetric, no need to consider switched vars
            vec![pos]
        } else {
            vec![pos, rpos]
        }
    }
}

struct Grid {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_corners() {
        for component in COMPONENTS {
            for corner in component.corners {
                assert!(corner.pos.0 == 0 || corner.pos.0 == component.width - 1);
                assert!(corner.pos.1 == 0 || corner.pos.1 == component.height - 1);
            }
        }
    }
}
