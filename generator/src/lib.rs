use std::fmt::{Display, Write};

use defaults::{COMPONENTS, Component};
use rand::{rng, seq::IteratorRandom};

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
                    })
                    .collect::<String>()
            ))?;
        }

        Ok(())
    }
}

const ANCHORS_5X5: [Anchor; 5] = [
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
    Anchor::Variable(2, 2, 1, 4),
];

fn try_place_component(rows: &Vec<Vec<CellState>>, comp: Component, x: u8, y: u8) {}

/// My general strategy is to generate the "anchors" of the board based on a pattern. E.g. in a 5x5 puzzle I generally want to have a 2x2 in each corner.
/// From this I can randomly select an anchor, randomly select the direction and component based on availability and config, and do this until we've run
/// out of room. The anchors can be pre-set, I can create a bunch of options and choose between them.
///
/// I could also create an anchor for the center, but without a set component. So I would specify that for a 5x5 the center is at 2, 2 (merged with the top
/// left corner anchor), and then choose between the components that would fit there for variety. I could make multiple for each grid size even. I'm realizing
/// that any algorithm that can size-independently generate grids is going to be eigher ugly, error-prone, or extraordinarily complictated.
fn generate_grid(width: u8, height: u8) -> Grid {
    // Only handle width, height >= 5, also ignore clues for now (will just implement a final pass over the puzzle to generate clues probably)
    let mut components = Vec::from(ANCHORS_5X5);
    let mut rows: Vec<Vec<CellState>> =
        // We leave an extra row and column for clues
        vec![vec![CellState::Uninitialized; width as usize + 1]; height as usize + 1];

    for component in components.iter() {
        if let Anchor::Set(sa) = component {
            for x in sa.x..(sa.x + sa.component.width) {
                for y in sa.y..(sa.y + sa.component.height) {
                    rows[y as usize][x as usize] = CellState::Fillable
                }
            }
        }
    }

    // Solidify anchor types
    components.iter_mut().for_each(|a| {
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

    loop {
        break;
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
