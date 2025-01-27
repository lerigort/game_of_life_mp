

// Structs and Implementations
#[derive(Clone, Debug)]
pub struct Cell {
    alive: bool,
    race: bool,
}

impl Cell {
    pub fn new(alive: bool, race: bool) -> Self {
        Self { alive, race }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn is_race_superior(&self) -> bool {
        self.race
    }
    pub fn set_state(&mut self, state: bool, race: bool){
        self.alive = state;
        self.race = race;
    }
}