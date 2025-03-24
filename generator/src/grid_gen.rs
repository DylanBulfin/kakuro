use std::{
    cell::Cell,
    collections::HashSet,
    fmt::{Display, Write},
};

use rand::{
    fill, rng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::anchors::{ANCHORS_8X8, ANCHORS_20X20};

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
        if self.width >= 5 {
            for x in 3..self.width as usize - 1 {
                cells[0][x] = GGCell::Blocked;
                cells[self.height as usize + 1][x] = GGCell::Blocked;
            }
        }

        if self.height >= 5 {
            for y in 3..self.height as usize - 1 {
                cells[y][0] = GGCell::Blocked;
                cells[y][self.width as usize + 1] = GGCell::Blocked;
            }
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
                    GGCell::Uninitialized => '_',
                    GGCell::Normal => 'O',
                    GGCell::Corner(_) => 'C',
                    GGCell::FusedCorner => 'F',
                    GGCell::Blocked => ',',
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
            (20, 20) => ANCHORS_20X20,
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
        let mut total_comps: Vec<(u8, u8, u8, u8)> = vec![];

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

            // TODO add random connector selection instead of always merging components
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
                        total_comps.push((ux, uy, *w, *h));
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

        if !grid.check_is_connected() {
            panic!("Grid not connected: {}", grid);
        }

        grid.fix_up();

        if grid
            .get_cages()
            .into_iter()
            .filter(|c| c.3 < 2 || c.3 > 9)
            .count()
            != 0
        {
            panic!("Unable to fix grid")
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
                    GGCell::Normal => {
                        if comp_cell != GGCell::Uninitialized {
                            return false;
                        }
                    }
                    GGCell::Uninitialized => {}
                    GGCell::Corner(dir) => {
                        if comp_cell != GGCell::Uninitialized
                            && comp_cell != GGCell::Corner(dir.get_opposite_dir())
                        {
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
                        if let GGCell::Corner(_) = comp_cell {
                            self.rows[iy][ix] = GGCell::FusedCorner
                        }
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

        let empty_cells: [(i8, i8); 2];
        let corner_cells: [(i8, i8); 3];

        match corner.dir {
            CornerDir::TopLeft => {
                corner_cells = [(-1, 0), (0, -1), (-1, -1)];
                empty_cells = [(-1, 1), (1, -1)];
            }
            CornerDir::TopRight => {
                corner_cells = [(1, 0), (0, -1), (1, -1)];
                empty_cells = [(1, 1), (-1, -1)];
            }
            CornerDir::BotLeft => {
                corner_cells = [(-1, 0), (0, 1), (-1, 1)];
                empty_cells = [(-1, -1), (1, 1)];
            }
            CornerDir::BotRight => {
                corner_cells = [(1, 0), (0, 1), (1, 1)];
                empty_cells = [(-1, 1), (1, -1)];
            }
        }

        let (x, y) = (corner.x as i8, corner.y as i8);

        for (dx, dy) in empty_cells {
            let cell = self.rows[(y + dy) as usize][(x + dx) as usize];

            if cell != GGCell::Uninitialized && cell != GGCell::Blocked {
                return true;
            }
        }

        for (dx, dy) in corner_cells {
            let cell = self.rows[(y + dy) as usize][(x + dx) as usize];

            if cell != GGCell::Uninitialized
                && cell != GGCell::Corner(corner.dir.get_opposite_dir())
            {
                return true;
            }
        }

        false
    }

    fn get_cell_fillable(&self, x: usize, y: usize) -> bool {
        if x as u8 >= self.width || y as u8 >= self.height {
            false
        } else {
            matches!(
                self.rows[y][x],
                GGCell::Normal | GGCell::Corner(_) | GGCell::FusedCorner
            )
        }
    }

    fn get_fillable_cell_count(&self) -> u32 {
        let mut count = 0;

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if self.get_cell_fillable(x, y) {
                    count += 1
                }
            }
        }

        count
    }

    fn check_is_connected(&self) -> bool {
        let fillable_cells = self.get_fillable_cell_count() as usize;

        let mut cell_queue: Vec<(usize, usize)> = Vec::new();
        let mut cell_set: HashSet<(usize, usize)> = HashSet::new();

        let mut curr_cell = (0, 0);
        cell_queue.push(curr_cell);
        cell_set.insert(curr_cell);

        loop {
            let last_curr_cell = curr_cell; // Save to check for change after

            // Try go left
            if curr_cell.0 > 0
                && !cell_set.contains(&(curr_cell.0 - 1, curr_cell.1))
                && self.get_cell_fillable(curr_cell.0 - 1, curr_cell.1)
            {
                let cell = (curr_cell.0 - 1, curr_cell.1);
                cell_set.insert(cell);
                cell_queue.push(cell);
                curr_cell = cell;
            }
            // Try go right
            else if curr_cell.0 < self.width as usize - 1
                && !cell_set.contains(&(curr_cell.0 + 1, curr_cell.1))
                && self.get_cell_fillable(curr_cell.0 + 1, curr_cell.1)
            {
                let cell = (curr_cell.0 + 1, curr_cell.1);
                cell_set.insert(cell);
                cell_queue.push(cell);
                curr_cell = cell;
            }
            // Try go up
            else if curr_cell.1 > 0
                && !cell_set.contains(&(curr_cell.0, curr_cell.1 - 1))
                && self.get_cell_fillable(curr_cell.0, curr_cell.1 - 1)
            {
                let cell = (curr_cell.0, curr_cell.1 - 1);
                cell_set.insert(cell);
                cell_queue.push(cell);
                curr_cell = cell;
            }
            // Try go down
            else if curr_cell.1 < self.height as usize - 1
                && !cell_set.contains(&(curr_cell.0, curr_cell.1 + 1))
                && self.get_cell_fillable(curr_cell.0, curr_cell.1 + 1)
            {
                let cell = (curr_cell.0, curr_cell.1 + 1);
                cell_set.insert(cell);
                cell_queue.push(cell);
                curr_cell = cell;
            }

            if last_curr_cell == curr_cell {
                // Reached a dead end (didn't update cell), try to backtrack
                if cell_queue.is_empty() {
                    // No cells to backtrack to
                    return false;
                } else {
                    curr_cell = cell_queue
                        .pop()
                        .expect("Unable to take element from non-empty list");
                }
            }

            // Check if we've reached the last cell
            if cell_set.len() == fillable_cells {
                return true;
            }
        }
    }

    fn get_cages(
        &self,
    ) -> Vec<(
        u8,    /*x*/
        u8,    /*y*/
        bool,  /*is_vertical*/
        usize, /*len*/
    )> {
        let mut res = Vec::new();

        // Check rows
        for y in 0..self.height {
            let mut x = 0u8;
            loop {
                if self.get_cell_fillable(x as usize, y as usize) {
                    // Found a new block, process it
                    let mut entry = (x, y, false, 1);
                    x += 1;

                    while self.get_cell_fillable(x as usize, y as usize) {
                        entry.3 += 1;
                        x += 1;
                    }

                    res.push(entry);
                } else if x < self.width - 1 {
                    x += 1;
                } else {
                    break;
                }
            }
        }
        for x in 0..self.width {
            let mut y = 0u8;
            loop {
                if self.get_cell_fillable(x as usize, y as usize) {
                    // Found a new block, process it
                    let mut entry = (x, y, true, 1);
                    y += 1;

                    while self.get_cell_fillable(x as usize, y as usize) {
                        entry.3 += 1;
                        y += 1;
                    }

                    res.push(entry);
                } else if y < self.height - 1 {
                    y += 1;
                } else {
                    break;
                }
            }
        }

        res
    }

    fn fix_up(&mut self) {
        // Get rid of any cages that are too long
        let mut cages = self.get_cages();
        let mut i = 0;

        // Fix any >9 length cages
        loop {
            let (x, y, is_vertical, len) = cages[i];
            if len > 9 {
                let midpoint: u8 = len as u8 / 2;

                for j in midpoint..len as u8 {
                    let (nx, ny) = if is_vertical { (x, y + j) } else { (x + j, y) };
                    let old_state = self.rows[ny as usize][nx as usize];

                    self.rows[ny as usize][nx as usize] = GGCell::Blocked;
                    if !self.check_is_connected() {
                        self.rows[ny as usize][nx as usize] = old_state;
                    } else {
                        break;
                    }
                }

                for j in (0..midpoint as u8).rev() {
                    let (nx, ny) = if is_vertical { (x, y + j) } else { (x + j, y) };
                    let old_state = self.rows[ny as usize][nx as usize];

                    self.rows[ny as usize][nx as usize] = GGCell::Blocked;
                    if !self.check_is_connected() {
                        self.rows[ny as usize][nx as usize] = old_state;
                    } else {
                        break;
                    }
                }
            }

            let new_cages = self.get_cages();

            if new_cages.len() != cages.len() {
                cages = new_cages;
                i = 0
            } else {
                i += 1;
                if i >= cages.len() {
                    break;
                }
            }
        }

        cages = self.get_cages();
        i = 0;

        // Fix any <2 length cages
        loop {
            let (x, y, is_vertical, len) = cages[i];
            if len < 2 {
                if is_vertical {
                    if y != 0 {
                        let (ux, uy) = (x as usize, y as usize - 1);
                        let old_state = self.rows[uy][ux];

                        self.rows[uy][ux] = GGCell::Normal;

                        if !self.check_is_connected()
                            || self.get_cages().iter().filter(|c| c.3 > 9).count() != 0
                        {
                            self.rows[uy][ux] = old_state;
                        } else {
                            cages = self.get_cages();
                            i = 0;
                            continue;
                        }
                    }
                    if y < self.height - 1 {
                        let (ux, uy) = (x as usize, y as usize + 1);
                        let old_state = self.rows[uy][ux];

                        self.rows[uy][ux] = GGCell::Normal;

                        if !self.check_is_connected()
                            || self.get_cages().iter().filter(|c| c.3 > 9).count() != 0
                        {
                            self.rows[uy][ux] = old_state;
                        } else {
                            cages = self.get_cages();
                            i = 0;
                            continue;
                        }
                    }
                } else {
                    if x != 0 {
                        let (ux, uy) = (x as usize - 1, y as usize);
                        let old_state = self.rows[uy][ux];

                        self.rows[uy][ux] = GGCell::Normal;

                        if !self.check_is_connected()
                            || self.get_cages().iter().filter(|c| c.3 > 9).count() != 0
                        {
                            self.rows[uy][ux] = old_state;
                        } else {
                            cages = self.get_cages();
                            i = 0;
                            continue;
                        }
                    }
                    if x < self.width - 1 {
                        let (ux, uy) = (x as usize + 1, y as usize);
                        let old_state = self.rows[uy][ux];

                        self.rows[uy][ux] = GGCell::Normal;

                        if !self.check_is_connected()
                            || self.get_cages().iter().filter(|c| c.3 > 9).count() != 0
                        {
                            self.rows[uy][ux] = old_state;
                        } else {
                            cages = self.get_cages();
                            i = 0;
                            continue;
                        }
                    }
                }
            }

            i += 1;
            if i >= cages.len() {
                break;
            }
        }
    }

    fn to_bool_vec(self) -> Vec<Vec<bool>> {
        self.rows
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|c| matches!(c, GGCell::Normal | GGCell::Corner(_) | GGCell::FusedCorner))
                    .collect()
            })
            .collect()
    }
}

pub fn get_grid(width: u8, height: u8) -> Vec<Vec<bool>> {
    //let grid = Grid::generate(8, 8);
    let grid = Grid::generate(width, height);

    println!("{}", grid);

    grid.to_bool_vec()
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
                GGCell::Uninitialized,
                GGCell::Blocked,
                GGCell::Uninitialized,
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
                GGCell::Uninitialized,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
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
                GGCell::Uninitialized,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Normal,
                GGCell::Uninitialized,
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
                GGCell::Uninitialized,
                GGCell::Blocked,
                GGCell::Uninitialized,
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

    #[test]
    fn test_volume() {
        // The grid generation doesn't need to be incredibly efficient but shouldn't take forever. If this starts
        // taking an annoying amount of time I should rethink the performance.
        for _ in 0..100 {
            let _grid = Grid::generate(20, 20);
        }
    }
}
