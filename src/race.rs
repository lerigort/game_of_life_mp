use crate::grid::Grid;
use std::ops::Fn;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaceType {
    Superior,
    Indoctrination,
    Necrodancer,
}

pub struct Race {
    pub race_type: Option<RaceType>,
    pub color: (f32, f32, f32, f32), // r g b a
    pub rules: Vec<Box<dyn Rule>>,
}

pub trait Rule {
    fn apply(&self, grid: &Grid, idx: usize) -> Option<RaceType>;
}

impl<F> Rule for F
where
    F: Fn(&Grid, usize) -> Option<RaceType> + 'static,
{
    fn apply(&self, grid: &Grid, idx: usize) -> Option<RaceType> {
        (self)(grid, idx)
    }
}

impl Race {
    pub fn new(race_type: Option<RaceType>, color: (f32, f32, f32, f32)) -> Self {
        Self {
            race_type,
            color, 
            rules: Vec::new(),
        }
    }
    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule)
    }

    pub fn apply_all_rules(&self, grid: &Grid, idx: usize) {
        for rule in &self.rules {
            rule.apply(grid, idx);
        }
    }
}

fn create_superior() -> Race {
    let mut race = Race::new(Some(RaceType::Superior), (1.0, 0.0, 0.0, 0.1));

    // Example rule: if 2 or more neighbors trigger Superior behavior
    race.add_rule(Box::new(|grid: &Grid, idx: usize| {
        //let neighbors = grid.get_neighbor_indices(idx);
        //let superior_count = neighbors.iter().filter(|&&neighbor_idx| grid.cells[neighbor_idx] == Some(RaceType::Superior)).count();
        //if superior_count >= 2 {
        //    Some(RaceType::Superior)
        //} else {
        //   None
        //}

        Some(RaceType::Superior)
    }));

    race
}
