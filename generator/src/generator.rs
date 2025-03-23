use std::fmt::{Display, Write};

use rand::{
    rng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::anchors::ANCHORS_8X8;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Component {
    pub(crate) width: u8,
    pub(crate) height: u8,
}

impl Component {
    pub(crate) const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    fn get_cells(&self) -> Vec<Vec<GGCell>> {
        // Cells contains one layer on each side for extra info
        let mut cells =
            vec![vec![GGCell::Uninitialized; self.width as usize + 2]; self.height as usize + 2];

        // Set corners
        cells[1][1] = GGCell::Corner(CornerDir::TopLeft);
        cells[self.height as usize][1] = GGCell::Corner(CornerDir::BotLeft);
        cells[1][self.width as usize] = GGCell::Corner(CornerDir::TopRight);
        cells[self.height as usize][self.width as usize] = GGCell::Corner(CornerDir::BotRight);

        // Set blocked cells
        for x in 2..self.width as usize {
            cells[0][x] = GGCell::Blocked;
            cells[self.height as usize + 1][x] = GGCell::Blocked;
        }

        for y in 2..self.height as usize {
            cells[y][0] = GGCell::Blocked;
            cells[y][self.width as usize + 1] = GGCell::Blocked;
        }

        // Set normal fillable cells
        for y in 1..=self.height as usize {
            for x in 1..=self.width as usize {
                if cells[y][x] == GGCell::Uninitialized {
                    cells[y][x] = GGCell::Normal
                }
            }
        }

        cells
    }

    // Position of top left corner of component
    fn get_pos_from_corner(&self, corner: Corner) -> (i8, i8) {
        let (x, y) = (corner.x as i8, corner.y as i8);
        let (w, h) = (self.width as i8, self.height as i8);
        match corner.dir {
            CornerDir::TopLeft => (x, y),
            CornerDir::TopRight => (x - w + 1, y),
            CornerDir::BotLeft => (x, y - h + 1),
            CornerDir::BotRight => (x - w + 1, y - h + 1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Anchor {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) component: Component,
}

impl Anchor {
    pub(crate) const fn new(x: u8, y: u8, component: Component) -> Self {
        Self { x, y, component }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CornerDir {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight,
}

impl CornerDir {
    fn get_opposite_dir(&self) -> Self {
        match self {
            Self::TopLeft => Self::BotRight,
            Self::TopRight => Self::BotLeft,
            Self::BotLeft => Self::TopRight,
            Self::BotRight => Self::TopLeft,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Corner {
    x: u8,
    y: u8,
    dir: CornerDir,
}

impl Corner {
    fn new(x: u8, y: u8, dir: CornerDir) -> Self {
        Self { x, y, dir }
    }

    fn generate_corners(x: u8, y: u8, component: Component) -> [Corner; 4] {
        [
            Corner::new(x, y, CornerDir::TopLeft),
            Corner::new(x + component.width - 1, y, CornerDir::TopRight),
            Corner::new(x, y + component.height - 1, CornerDir::BotLeft),
            Corner::new(
                x + component.width - 1,
                y + component.height - 1,
                CornerDir::BotRight,
            ),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GGCell {
    Uninitialized,
    Normal,
    Corner(CornerDir),
    FusedCorner,
    Blocked,
}

struct Grid {
    width: u8,
    height: u8,
    rows: Vec<Vec<GGCell>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                f.write_char(match cell {
                    GGCell::Uninitialized => '?',
                    GGCell::Normal => '_',
                    GGCell::Corner(_) => 'C',
                    GGCell::FusedCorner => 'F',
                    GGCell::Blocked => 'X',
                })?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Grid {
    fn generate(width: u8, height: u8) -> Self {
        let rows = vec![vec![GGCell::Uninitialized; width as usize]; height as usize];

        let mut grid = Self {
            width,
            height,
            rows,
        };

        let anchors = match (width, height) {
            (8, 8) => ANCHORS_8X8,
            _ => panic!(),
        };

        let mut active_corners: Vec<Corner> = Vec::new();

        // Initialize anchors
        for anchor in anchors {
            active_corners.append(
                &mut Corner::generate_corners(anchor.x, anchor.y, anchor.component).to_vec(),
            );
            grid.try_place_component(anchor.x, anchor.y, anchor.component);
        }

        let rng = &mut rng();

        let mut wasted_iters = 0;

        loop {
            // Check for dead corners
            active_corners = active_corners
                .into_iter()
                .filter(|c| !grid.corner_is_dead(c))
                .collect();

            if active_corners.is_empty() {
                break;
            } else {
                //println!("{}", active_corners.len())
            }

            // Select a random corner to try to build from
            let rcorn = active_corners.choose(rng).unwrap_or(&active_corners[0]);

            // Try to place new component
            let max_w = (width / 4).min(2);
            let max_h = (height / 4).min(2);

            let mut sorted_ws: Vec<_> = (2..=max_w).collect();
            let mut sorted_hs: Vec<_> = (2..=max_h).collect();

            sorted_hs.shuffle(rng);
            sorted_ws.shuffle(rng);

            // Used named loop to break from double loop easily
            'outer: for h in sorted_hs.iter() {
                for w in sorted_ws.iter() {
                    let component = Component::new(*w, *h);
                    let opp_corner = Corner::new(rcorn.x, rcorn.y, rcorn.dir.get_opposite_dir());
                    let pos = component.get_pos_from_corner(opp_corner);

                    if pos.0 < 0 || pos.1 < 0 {
                        continue;
                    }

                    let (ux, uy) = (pos.0 as u8, pos.1 as u8);

                    if grid.try_place_component(ux, uy, component) {
                        // Update active corners
                        active_corners.append(
                            &mut Corner::generate_corners(ux, uy, component)
                                .into_iter()
                                .collect(),
                        );
                        wasted_iters = -1;
                        break 'outer;
                    }
                }
            }

            wasted_iters += 1;

            if wasted_iters > 100 {
                println!("{:?}", active_corners);
                break;
            }
        }

        grid
    }

    fn try_place_component(&mut self, cx: u8, cy: u8, component: Component) -> bool {
        if cx + component.width > self.width || cy + component.height > self.height {
            return false;
        }
        // Lower and upper bounds of index into component grid cells (e.g. if the component is being placed at x = 0, we don't
        // want to check the leftmost column in the component cells)
        let lower_x = if cx == 0 { 1 } else { 0 };
        let lower_y = if cy == 0 { 1 } else { 0 };

        // Inclusive
        let upper_x = if cx + component.width == self.width {
            component.width
        } else {
            component.width + 1
        };
        let upper_y = if cy + component.height == self.height {
            component.height
        } else {
            component.height + 1
        };

        let cells = component.get_cells();

        // Check for conflicts, don't update cells yet
        for y in lower_y..=upper_y {
            for x in lower_x..=upper_x {
                let comp_cell = cells[y as usize][x as usize];
                let (ix, iy) = ((cx + x - 1) as usize, (cy + y - 1) as usize);

                match &self.rows[iy][ix] {
                    GGCell::Normal => return false,
                    GGCell::Uninitialized => {}
                    GGCell::Corner(dir) => {
                        if comp_cell != GGCell::Uninitialized
                            && comp_cell != GGCell::Corner(dir.get_opposite_dir())
                        {
                            if component.width == 2 && component.height == 2 {
                                println!(
                                    "{} {} {} {} {} {} {:?} {:?} {:?} {:?}",
                                    cx,
                                    cy,
                                    x,
                                    y,
                                    ix,
                                    iy,
                                    upper_x,
                                    upper_y,
                                    self.rows[iy][ix],
                                    comp_cell
                                );
                            }
                            return false;
                        }
                    }
                    GGCell::FusedCorner | GGCell::Blocked => {
                        if comp_cell != GGCell::Uninitialized {
                            return false;
                        }
                    }
                }
            }
        }

        for y in lower_y..=upper_y {
            for x in lower_x..=upper_x {
                let comp_cell = cells[y as usize][x as usize];
                let (ix, iy) = ((cx + x - 1) as usize, (cy + y - 1) as usize);

                match &self.rows[iy][ix] {
                    GGCell::Uninitialized => self.rows[iy][ix] = comp_cell,
                    GGCell::Corner(_) => {
                        // We already checked that corners match
                        self.rows[iy][ix] = GGCell::FusedCorner
                    }
                    _ => {} // Any other cells should not be overwritten,
                }
            }
        }

        true
    }

    fn corner_is_dead(&self, corner: &Corner) -> bool {
        // Check for corner on the edges, these are always dead
        if corner.x == 0
            || corner.x >= self.width - 1
            || corner.y == 0
            || corner.y >= self.height - 1
        {
            return true;
        }

        let empty_cells: [(i8, i8); 5];

        match corner.dir {
            CornerDir::TopLeft => {
                empty_cells = [(-1, 0), (0, -1), (-1, -1), (-1, 1), (1, -1)];
            }
            CornerDir::TopRight => {
                empty_cells = [(1, 0), (0, -1), (1, -1), (1, 1), (-1, -1)];
            }
            CornerDir::BotLeft => {
                empty_cells = [(-1, 0), (0, 1), (-1, 1), (-1, -1), (1, 1)];
            }
            CornerDir::BotRight => {
                empty_cells = [(1, 0), (0, 1), (1, 1), (-1, 1), (1, -1)];
            }
        }

        let (x, y) = (corner.x as i8, corner.y as i8);

        for (dx, dy) in empty_cells {
            let cell = self.rows[(y + dy) as usize][(x + dx) as usize];

            if cell != GGCell::Uninitialized {
                return true;
            }
        }

        false
    }
}

pub fn start() {
    let grid = Grid::generate(8, 8);

    println!("{}", grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_get_cells() {
        let component = Component::new(5, 5);

        let expected = [
            [
                GGCell::Uninitialized,
                GGCell::Uninitialized,
                GGCell::Blocked,
                GGCell::Blocked,
                GGCell::Blocked,
                GGCell::Uninitialized,
                GGCell::Uninitialized,
            ],
            [
                GGCell::Uninitialized,
                GGCell::Corner(CornerDir::TopLeft),
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Corner(CornerDir::TopRight),
                GGCell::Uninitialized,
            ],
            [
                GGCell::Blocked,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Blocked,
            ],
            [
                GGCell::Blocked,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Blocked,
            ],
            [
                GGCell::Blocked,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Blocked,
            ],
            [
                GGCell::Uninitialized,
                GGCell::Corner(CornerDir::BotLeft),
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Corner(CornerDir::BotRight),
                GGCell::Uninitialized,
            ],
            [
                GGCell::Uninitialized,
                GGCell::Uninitialized,
                GGCell::Blocked,
                GGCell::Blocked,
                GGCell::Blocked,
                GGCell::Uninitialized,
                GGCell::Uninitialized,
            ],
        ];

        assert_eq!(component.get_cells(), expected)
    }

    #[test]
    fn test_get_pos_from_corner() {
        let component = Component::new(5, 5);
        let corner1 = Corner::new(3, 8, CornerDir::BotLeft);
        let corner2 = Corner::new(9, 8, CornerDir::BotRight);

        let res1 = component.get_pos_from_corner(corner1);
        let res2 = component.get_pos_from_corner(corner2);

        assert_eq!(res1, (3, 4));
        assert_eq!(res2, (5, 4));
    }

    #[test]
    fn test_generate_corners() {
        let component = Component::new(5, 5);

        let res = Corner::generate_corners(3, 3, component);
        let expected = [
            Corner::new(3, 3, CornerDir::TopLeft),
            Corner::new(7, 3, CornerDir::TopRight),
            Corner::new(3, 7, CornerDir::BotLeft),
            Corner::new(7, 7, CornerDir::BotRight),
        ];

        assert_eq!(res, expected);
    }
}
