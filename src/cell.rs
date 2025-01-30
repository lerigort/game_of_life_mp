use crate::race::RaceType;

// Structs and Implementations
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    alive: bool,
    race_type: Option<RaceType>,
}

impl Cell {
    pub fn new(alive: bool, race_type: Option<RaceType>) -> Self {
        Self { alive, race_type }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn get_race(&self) -> Option<RaceType> {
        self.race_type
    }
    pub fn set_state(&mut self, state: bool, race_type: Option<RaceType>){
        self.alive = state;
        self.race_type = race_type;
    }
}