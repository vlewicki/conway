#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};

use crate::conway::Cell::{Alive, Dead};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: grid.into(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value
    }

    pub fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let r0 = row.saturating_sub(1);
        let r1 = (row + 2).min(self.rows);
        let c0 = col.saturating_sub(1);
        let c1 = (col + 2).min(self.cols);

        (r0..r1).flat_map(move |r| {
            (c0..c1).filter_map(move |c| {
                if r == row && c == col {
                    None
                } else {
                    Some((r, c))
                }
            })
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        let newval = match self.grid.get(row, col) {
            Alive => Dead,
            Dead => Alive,
        };
        self.grid.set(newval, row, col);
    }

    pub fn step(&mut self) {
        let mut new_grid = Grid::<Cell>::new(self.grid.rows, self.grid.cols);
        for r in 0..self.grid.rows {
            for c in 0..self.grid.cols {
                let alive_neighbors_count = self
                    .grid
                    .neighbours(r, c)
                    .map(|x| self.grid.get(x.0, x.1))
                    .filter(|&&x| x == Cell::Alive)
                    .count();
                let new_cell_state = match (self.grid.get(r, c), alive_neighbors_count) {
                    (Cell::Alive, 2..=3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };
                new_grid.set(new_cell_state, r, c);
            }
        }
        self.grid = new_grid
    }
}
