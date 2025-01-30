use crate::grid::Grid;
use std::{ops::Fn, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaceType {
    Superior,
    Indoctrination,
    Necrodancer,
}

pub struct Race {
    pub race_type: Option<RaceType>,
    pub color: (f32, f32, f32, f32), // r g b a
    pub rules: Vec<Arc<dyn Rule>>,
}

pub trait Rule: Send + Sync {
    fn apply(&self, grid: &Grid, idx: usize) -> Option<RaceType>;
}

impl<F> Rule for F
where
    F: Fn(&Grid, usize) -> Option<RaceType> + 'static + Send + Sync,
{
    fn apply(&self, grid: &Grid, idx: usize) -> Option<RaceType> {
        (self)(grid, idx)
    }
}
// There is absolutely NO need in cloning, every race is singular and struct with race exist in single variant
// i just WANTED to be able to clone it. Thought it never used.
impl Clone for Race {
    fn clone(&self) -> Self {
        Race {
            race_type: self.race_type,
            color: self.color,
            rules: self.rules.clone(),
        }
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
    pub fn add_rule(&mut self, rule: Arc<dyn Rule>) {
        self.rules.push(rule)
    }

    pub fn apply_all_rules(&self, grid: &Grid, idx: usize) {
        for rule in &self.rules {
            rule.apply(grid, idx);
        }
    }

    pub fn create_superior() -> Race {
        let mut race = Race::new(Some(RaceType::Superior), (1.0, 0.0, 0.0, 0.1));
    
        // Rule: if 2 or more neighbors have Superior behavior
        race.add_rule(Arc::new(|grid: &Grid, idx: usize| {
            let superior_count = grid
                .get_neighbor_coords(idx)
                .iter()
                .filter_map(|&coords| Some(grid.coords_to_index(coords)))
                .filter(|&neighbor_idx| grid.cells[neighbor_idx].get_race() == Some(RaceType::Superior))
                .count();
            (superior_count >= 2).then(|| RaceType::Superior)
        }));
    
        race
    }

    pub fn create_indoctrinator() -> Race {
        // for the time being its alomost the same, as the Superior

        let mut race = Race::new(Some(RaceType::Indoctrination), (0.0, 1.0, 0.0, 0.1));
    
        // Rule: if 2 or more neighbors have Superior behavior
        race.add_rule(Arc::new(|grid: &Grid, idx: usize| {
            let superior_count = grid
                .get_neighbor_coords(idx)
                .iter()
                .filter_map(|&coords| Some(grid.coords_to_index(coords)))
                .filter(|&neighbor_idx| grid.cells[neighbor_idx].get_race() == Some(RaceType::Indoctrination))
                .count();
            (superior_count >= 2).then(|| RaceType::Indoctrination)
        }));
    
        race
    }
}


