use std::usize;

use crate::{cell::Cell, race::{Race, RaceType}};
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
            cells: vec![Cell::new(false, None); width * height],
        }
    }

    pub fn get_neighbor_coords(&self, idx: usize) -> Vec<(usize, usize)> {
        let mut neighbors_idx = Vec::new();
        let (x, y) = self.index_to_coords(idx);

        let neighbor_offsets = [
            (-1, 1), (0, 1), (1, 1),
            (-1, 0),         (1, 0),
            (-1, -1), (0, -1), (1, -1),
        ];
    
        for (x_shift, y_shift) in neighbor_offsets.iter() {
            let x_neigh = x as isize + x_shift;
            let y_neigh = y as isize + y_shift;
    
            if x_neigh >= 0 && (x_neigh as usize) < self.width &&
               y_neigh >= 0 && (y_neigh as usize) < self.height {
                neighbors_idx.push((x_neigh as usize, y_neigh as usize));
            }
        }
    
        neighbors_idx
    }
    
    
    pub fn set_state(&mut self, cells_coords: &[(usize, usize)], is_init: bool, race_type: Option<RaceType>) {
        if is_init {
            // at first iteration creates a field of dead cells
            self.cells = vec![Cell::new(false, None); self.width * self.height];
        }
        for &pos in cells_coords.iter() {
            let idx = self.coords_to_index(pos);
            self.cells[idx].set_state(true, race_type);
        }
    }

    fn cell_next_state(&self, cell_idx: usize) -> (bool, Option<RaceType>) {
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
                    if self.cells[idx].get_race() == Some(RaceType::Superior) {
                        num_superior_race += 1;
                    }
                }
            }
        }

        // Rules (from wikipedia)
        if cell.is_alive() && (num_neighbour_alive == 2 || num_neighbour_alive == 3) {
            return (true, cell.get_race()); // alive, same race
        }

        if !cell.is_alive() && num_neighbour_alive == 3 {
            if num_superior_race >= 2 {
                return (true, Some(RaceType::Superior)); // alive, superior race
            }
            return (true, Some(RaceType::Indoctrination)); // alive.
        }
        (false, None)
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
            .collect::<Vec<(bool, Option<RaceType>)>>();

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
