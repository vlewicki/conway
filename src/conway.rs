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
            grid: grid.into()
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row*self.cols+col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row*self.cols+col] = value
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut x = vec![];
        for r in row.saturating_sub(1)..self.rows.min(row+2){
            for c in col.saturating_sub(1)..self.cols.min(col+2){
                if r == row && c == col {
                    continue;
                }
                x.push((r, c));
            }
        }
        x
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(Default, Serialize, Deserialize)]
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
    
    pub fn toggle(&mut self,row: usize, col: usize) {
        let newval = match self.grid.get(row, col){ Alive => Dead, Dead => Alive};
        self.grid.set(newval, row, col);
    }

    pub fn step(&mut self) {
        let mut new_grid = Grid::<Cell>::new(self.grid.rows, self.grid.cols);
        for r in 0..self.grid.rows {
            for c in 0..self.grid.cols {
                let alive_neighbors_count = self.grid.neighbours(r, c).iter()
                                        .map(|x| self.grid.get(x.0, x.1))
                                        .filter(|&&x| x == Cell::Alive).count();
                let new_cell_state = match (self.grid.get(r,c), alive_neighbors_count) {
                        (Cell::Alive, 2..=3) => Cell::Alive,
                        (Cell::Alive, _) => Cell::Dead,
                        (Cell::Dead, 3) => Cell::Alive,
                        (Cell::Dead, _) => Cell::Dead
                    };
                new_grid.set(new_cell_state, r, c);
            }
        }
        self.grid = new_grid
    }
}
