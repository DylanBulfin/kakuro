use std::fmt::{Display, Write};

use defaults::{COMPONENTS, Component, Corner, CornerDir};
use rand::{
    rng,
    seq::{IndexedRandom, IteratorRandom},
};

mod defaults;

struct SetAnchor {
    x: u8,
    y: u8,
    component: Component,
}

enum Anchor {
    Set(SetAnchor),
    Variable(u8, u8, usize, usize), // x, y, min_component, max_component
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Uninitialized,
    Corner,

    Fillable,
    Rule,
    Block,
}

struct Grid {
    width: u8,
    height: u8,
    rows: Vec<Vec<CellState>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            f.write_fmt(format_args!(
                "{}\n",
                self.rows[i as usize]
                    .iter()
                    .map(|s| match s {
                        CellState::Fillable => '_',
                        CellState::Rule => 'R',
                        CellState::Block => 'X',
                        CellState::Uninitialized => '?',
                        CellState::Corner => 'C',
                    })
                    .collect::<String>()
            ))?;
        }

        Ok(())
    }
}

const ANCHORS_5X5: [Anchor; 4] = [
    Anchor::Set(SetAnchor {
        x: 1,
        y: 1,
        component: COMPONENTS[0],
    }),
    Anchor::Set(SetAnchor {
        x: 4,
        y: 1,
        component: COMPONENTS[0],
    }),
    Anchor::Set(SetAnchor {
        x: 1,
        y: 4,
        component: COMPONENTS[0],
    }),
    Anchor::Set(SetAnchor {
        x: 4,
        y: 4,
        component: COMPONENTS[0],
    }),
    //Anchor::Variable(2, 2, 1, 4),
];

fn try_place_component(rows: &mut Vec<Vec<CellState>>, comp: &Component, x: u8, y: u8) -> bool {
    let mut changed_cells: Vec<(u8, u8)> = Vec::new();
    let mut valid: bool = true;

    if x + comp.width > rows[0].len() as u8 || y + comp.height > rows.len() as u8 {
        return false;
    }

    for row in y..y + comp.height {
        for col in x..x + comp.width {
            if rows[row as usize][col as usize] == CellState::Fillable {
                valid = false;
                break;
            }
            if rows[row as usize][col as usize] == CellState::Uninitialized {
                rows[row as usize][col as usize] = CellState::Fillable;
                changed_cells.push((col, row));
            }
        }
        if !valid {
            break;
        }
    }

    if !valid {
        // Restore previous state
        for cell in changed_cells {
            rows[cell.1 as usize][cell.0 as usize] = CellState::Uninitialized;
        }
        false
    } else {
        for corner in comp.corners {
            rows[(corner.pos.1 + y) as usize][(corner.pos.0 + x) as usize] = CellState::Corner;
        }

        true
    }
}

fn corner_is_finished(rows: &Vec<Vec<CellState>>, comp_x: u8, comp_y: u8, corner: Corner) -> bool {
    let pos = (comp_x + corner.pos.0, comp_y + corner.pos.1);
    let (width, height) = (rows[0].len() as u8, rows.len() as u8);

    // These are cells that can be empty or corners and still allow a 2x2 component merged with this corner
    let check_cells: [(i8, i8); 5];

    // Check if it's too far in one direction
    match corner.dir {
        CornerDir::TopLeft => {
            if pos.0 < 2 || pos.1 < 2 {
                return true;
            }
            check_cells = [(-1, 0), (0, -1), (-1, -1), (-1, 1), (1, -1)];
        }
        CornerDir::TopRight => {
            if pos.0 > width - 2 || pos.1 < 2 {
                return true;
            }
            check_cells = [(1, 0), (0, -1), (1, -1), (-1, -1), (1, 1)];
        }
        CornerDir::BotLeft => {
            if pos.0 < 2 || pos.1 > height - 2 {
                return true;
            }
            check_cells = [(-1, 0), (0, 1), (-1, 1), (-1, -1), (1, 1)];
        }
        CornerDir::BotRight => {
            if pos.0 > width - 2 || pos.1 > height - 2 {
                return true;
            }
            check_cells = [(1, 0), (0, 1), (1, 1), (-1, 1), (1, -1)];
        }
    }

    for cell in check_cells {
        let (cell_x, cell_y) = (pos.0 as i8 + cell.0, pos.1 as i8 + cell.1);

        if rows[cell_y as usize][cell_x as usize] != CellState::Uninitialized {
            return true;
        }
    }

    false
}

/// My general strategy is to generate the "anchors" of the board based on a pattern. E.g. in a 5x5 puzzle I generally want to have a 2x2 in each corner.
/// From this I can randomly select an anchor, randomly select the direction and component based on availability and config, and do this until we've run
/// out of room. The anchors can be pre-set, I can create a bunch of options and choose between them.
///
/// I could also create an anchor for the center, but without a set component. So I would specify that for a 5x5 the center is at 2, 2 (merged with the top
/// left corner anchor), and then choose between the components that would fit there for variety. I could make multiple for each grid size even. I'm realizing
/// that any algorithm that can size-independently generate grids is going to be eigher ugly, error-prone, or extraordinarily complictated.
fn generate_grid(width: u8, height: u8) -> Grid {
    // Only handle width, height >= 5, also ignore clues for now (will just implement a final pass over the puzzle to generate clues probably)
    let mut active_components = Vec::from(ANCHORS_5X5);
    let mut rows: Vec<Vec<CellState>> =
        // We leave an extra row and column for clues
        vec![vec![CellState::Uninitialized; width as usize + 1]; height as usize + 1];

    // Solidify anchor types
    active_components.iter_mut().for_each(|a| {
        if let Anchor::Variable(x, y, min_i, max_i) = a {
            {
                let i = (*min_i..=*max_i).choose(&mut rng()).unwrap_or(*max_i); // Default to largest size
                *a = Anchor::Set(SetAnchor {
                    x: *x,
                    y: *y,
                    component: COMPONENTS[i],
                })
            }
        }
    });

    for component in active_components.iter() {
        if let Anchor::Set(sa) = component {
            for x in sa.x..(sa.x + sa.component.width) {
                for y in sa.y..(sa.y + sa.component.height) {
                    if rows[y as usize][x as usize] != CellState::Corner {
                        rows[y as usize][x as usize] = CellState::Fillable
                    }
                }
            }

            for corner in sa.component.corners {
                rows[(corner.pos.1 + sa.y) as usize][(corner.pos.0 + sa.x) as usize] =
                    CellState::Corner;
            }
        }
    }

    loop {
        // Prune "dead" components
        let mut i = 0;
        loop {
            if i >= active_components.len() {
                break;
            }

            if let Anchor::Set(sa) = &active_components[i] {
                if sa
                    .component
                    .corners
                    .map(|c| corner_is_finished(&rows, sa.x, sa.y, c))
                    .iter()
                    .fold(true, |a, &b| a && b)
                {
                    active_components.swap_remove(i);
                }
            } else {
                panic!("Variable anchor after they should be gone ")
            }

            i += 1;
        }

        if active_components.is_empty() {
            break;
        }

        let rng = &mut rng();
        // Try to add a new component to a random corner
        let rand_comp = active_components
            .choose(rng)
            .unwrap_or(&active_components[0]);
        let (rand_corn, pos) = if let Anchor::Set(sa) = rand_comp {
            (
                sa.component
                    .corners
                    .choose(rng)
                    .unwrap_or(&sa.component.corners[0]),
                (sa.x, sa.y),
            )
        } else {
            panic!("Variable anchor after they should be gone ")
        };

        for component in COMPONENTS.iter().rev() {
            if try_place_component(&mut rows, component, pos.0, pos.1) {
                break;
            }
        }
    }

    Grid {
        width: width + 1,
        height: height + 1,
        rows,
    }
}

pub fn testing() {
    println!("{}", generate_grid(5, 5))
}
