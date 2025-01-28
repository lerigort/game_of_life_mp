use std::usize;

use crate::cell::Cell;
use rand::Rng;
use rayon::prelude::*;

pub struct Grid {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    // Width and height of the Grid
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(false, false); width * height],
        }
    }
    
    pub fn set_state(&mut self, cells_coords: &[(usize, usize)], is_init: bool, is_superior_race: bool) {
        if is_init {
            // at first iteration creates a field of dead cells
            self.cells = vec![Cell::new(false, false); self.width * self.height];
        }
        for &pos in cells_coords.iter() {
            let idx = self.coords_to_index(pos);
            self.cells[idx].set_state(true, is_superior_race);
        }
    }

    fn cell_next_state(&self, cell_idx: usize) -> (bool, bool) {
        let cell = self.cells[cell_idx].clone();
        let (cell_x, cell_y) = self.index_to_coords(cell_idx);
        // Check boundaries and add neighgours
        let mut num_neighbour_alive = 0;
        let mut num_superior_race = 0;
        for &x_off in [-1, 0, 1].iter() {
            for &y_off in [-1, 0, 1].iter() {
                if x_off == 0 && y_off == 0 {
                    continue;
                }
                
                let (neighbour_x,  neighbour_y)= (cell_x as isize + x_off, cell_y as isize + y_off);
                if neighbour_x < 0
                    || neighbour_x > self.width as isize - 1
                    || neighbour_y < 0
                    || neighbour_y > self.height as isize - 1
                {
                    continue;
                }
                let idx =
                    self.coords_to_index((neighbour_x as usize, neighbour_y as usize));
                if self.cells[idx].is_alive() {
                    num_neighbour_alive += 1;
                    if self.cells[idx].is_race_superior() {
                        num_superior_race += 1;
                    }
                }
            }
        }

        // Rules (from wikipedia)
        if cell.is_alive() && (num_neighbour_alive == 2 || num_neighbour_alive == 3) {
            return (true, cell.is_race_superior()); // alive, same race
        }
        if !cell.is_alive() && num_neighbour_alive == 3 {
            if num_superior_race >= 2 {
                return (true, true); // alive, superior race
            }
            return (true, false); // alive.
        }
        (false, false) // hes DEAAAAD
    }

    pub fn update(&mut self) {
        // Vector of next states. It will match by index
        // Get next states
        // Iterative lags, parallel stronk
        // let mut next_states = vec![false; self.cells.len()];
        // for idx in (0..self.cells.len()) {
        //     let next_state = self.cell_next_state(idx);
        //     next_states[idx] = next_state;
        // }
        let next_states = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| {
                // next state
                self.cell_next_state(idx)
            })
            .collect::<Vec<(bool, bool)>>();

        // Update states
        // for idx in 0..self.cells.len() {
        //     self.cells[idx].alive = next_states[idx];
        // }
        self.cells = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| Cell::new(next_states[idx].0, next_states[idx].1  ))
            .collect::<Vec<Cell>>();
    }
    /// Converts a pair of cell coords to index in the cells vector
    pub fn coords_to_index(&self, (x, y): (usize, usize)) -> usize {
        return y * self.width + x
    }

    /// Converts a index in the cells vecotr into pair of cell coords
    pub fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.height, index / self.width)
    }
}
