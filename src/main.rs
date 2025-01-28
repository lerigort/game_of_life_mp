mod cell;
mod grid;

use crate::grid::Grid;
use clap::{App, Arg};

use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

const GRID: bool = false;
//const CELL_SIZE: f32 = SCREEN_SIZE.0 / GRID_WIDTH as f32;
const STARTING_CELLS: usize = 10;

#[allow(dead_code)]
const BLINKER: [(usize, usize); 3] = [(4, 4), (4, 5), (4, 6)];
#[allow(dead_code)]
const TOAD: [(usize, usize); 6] = [(4, 4), (4, 5), (4, 6), (5, 3), (5, 4), (5, 5)];
#[allow(dead_code)]
const GLIDER: [(usize, usize); 5] = [(1, 2), (3, 2), (2, 3), (3, 3), (2, 4)];
#[allow(dead_code)]
const GLIDER_GUN: [(usize, usize); 36] = [
    (5, 1),
    (5, 2),
    (6, 1),
    (6, 2),
    (5, 11),
    (6, 11),
    (7, 11),
    (4, 12),
    (3, 13),
    (3, 14),
    (8, 12),
    (9, 13),
    (9, 14),
    (6, 15),
    (4, 16),
    (5, 17),
    (6, 17),
    (7, 17),
    (6, 18),
    (8, 16),
    (3, 21),
    (4, 21),
    (5, 21),
    (3, 22),
    (4, 22),
    (5, 22),
    (2, 23),
    (6, 23),
    (1, 25),
    (2, 25),
    (6, 25),
    (7, 25),
    (3, 35),
    (4, 35),
    (3, 36),
    (4, 36),
];

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: String,
}

struct MainState {
    paused: bool,
    grid: Grid,
    config: Config,
    last_click_pos: Option<(f32, f32)>,
}

impl MainState {
    pub fn new(_ctx: &mut Context, config: Config) -> Self {
        // Initialize the grid based on configuration
        let mut grid = Grid::new(config.grid_width, config.grid_height);
        // Initialize starting configuration
        let mut start_cells_coords: Vec<(usize, usize)> = vec![];
        match &config.initial_state[..] {
            "glider-gun" => {
                start_cells_coords = GLIDER_GUN.iter().map(|&p| p.into()).collect::<Vec<(usize, usize)>>();
            }
            "toad" => {
                start_cells_coords = TOAD.iter().map(|&p| p.into()).collect::<Vec<(usize, usize)>>();
            }
            "glider" => {
                start_cells_coords = GLIDER.iter().map(|&p| p.into()).collect::<Vec<(usize, usize)>>();
            }
            "blinker" => {
                start_cells_coords = BLINKER.iter().map(|&p| p.into()).collect::<Vec<(usize, usize)>>();
            }
            _ => {
                let mut rng = rand::thread_rng();
                for i in 0..config.grid_width{
                    for j in 0..config.grid_height{
                        if rng.gen_range(0..100) < 10 {
                            // start_cells_coords.push((i, j).into());
                        }
                    }
                }
            }
        }
        // Convert the starting states into a vector of points
        grid.set_state(&start_cells_coords, true, true); // creates a filed of dead cellz
        MainState {
            paused: true, 
            grid,
            config,
            last_click_pos: None
        }
    }

    fn pixel_to_grid(&self, pixel_x: f32, pixel_y: f32) -> (usize, usize) {

        // Get screen dimensions and grid dimensions
        let screen_size = self.config.screen_size;
        let grid_size = (self.config.grid_width, self.config.grid_height);

        // Calculate tile size
        let tile_size_x = screen_size.0 / grid_size.0 as f32;
        let tile_size_y = screen_size.1 / grid_size.1 as f32;

        // Map pixel coordinates to grid coordinates
        let grid_x = (pixel_x / tile_size_x).floor() as usize;
        let grid_y = (pixel_y / tile_size_y).floor() as usize;

        (grid_x, grid_y)
    }
    fn cell_tracking (&self) -> (usize, usize, usize) {
        // returns how many cells: alive, 1 race, 2nd race
        let mut alive = 0;
        let mut superior_race = 0;
        let mut other_race = 0;
        for cell in &self.grid.cells {
            if cell.is_alive() && cell.is_race_superior() {
                superior_race += 1
            }
            else if cell.is_alive() {
                other_race += 1 
            }
            else {
                continue;
            }
            alive += 1;
        }
        (alive, superior_race, other_race)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused {
            // Game is paused; skip update logic.
            return Ok(());
        }

        while ggez::timer::check_update_time(ctx, self.config.fps) {
            self.grid.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        // Mesh builder
        let mut builder = graphics::MeshBuilder::new();
        // Init, otherwise doesn't work for some reason
        builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., 0., 0.),
            graphics::BLACK,
        );
        // Draw cells
        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if cell.is_alive() {
                let mut color = graphics::Color::new(0., 200., 0., 1.); // Green
                if !cell.is_race_superior() {
                    color = graphics::Color::new(200., 200., 0., 1.); // Yellow
                }
            
                let (x, y) = self.grid.index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        x as f32 * self.config.cell_size,
                        y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                );
            }
        }
        // Draw grid
        if GRID {
            for idx in 0..self.grid.cells.len() {
                let color = graphics::Color::new(10., 10., 10., 1.); // ?
                let (x, y) = self.grid.index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::stroke(1.),
                    graphics::Rect::new(
                        x as f32 * self.config.cell_size,
                        y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                );
            }
        }
        let mesh = builder.build(ctx)?;
        // Draw
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // Present on screen
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Space {
            // Toggle pause state.
            self.paused = !self.paused;
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            let (alive, _, _) = self.cell_tracking();

            self.last_click_pos = Some((x, y));
            println!("Mouse clicked at: ({}, {})", x, y);

            // Convert pixel coordinates to grid coordinates
            let (grid_x, grid_y) = self.pixel_to_grid(x, y);
            println!("Clicked grid tile: ({}, {})", grid_x, grid_y);

            let mut superior_race = true;
            match alive {
                0..10 => superior_race = true,
                10..20 => superior_race = false,
                20.. => self.paused = false, 
            }
            self.grid.set_state(&vec![(grid_x, grid_y)], false, superior_race);

            let (alive, num_superior_race, num_other_race) = self.cell_tracking();
            println!("alive: {alive}, superior_race: {num_superior_race}, other_race: {num_other_race}");
        }
    }
}

fn main() -> GameResult {
    // CLI
    let matches = App::new("Game of Life")
        .version("0.1")
        .author("Zademn")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Grid width")
                .value_name("width")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Grid height")
                .value_name("height")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("initial_state")
                .short("s")
                .long("initial-state")
                .help("Initial state options: blinker, toad, glider, glider-gun, random")
                .value_name("initial_state")
                .takes_value(true)
                .required(false)
                .default_value("random"),
        )
        .get_matches();

    // Get Configurations
    let grid_width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let grid_height = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let initial_state = matches.value_of("initial_state").unwrap();
    let screen_size = (720., 720.);
    let fps = 10;
    // Set configuration
    let config: Config = Config {
        grid_width,
        grid_height,
        cell_size: screen_size.0 / grid_width as f32,
        screen_size,
        fps,
        initial_state: initial_state.to_string(),
    };

    // Setup ggez stuff
    let cb = ContextBuilder::new("Game of life", "Zademn")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (ctx, event_loop) = &mut cb.build()?; // `?` because the build function may fail
    graphics::set_window_title(ctx, "Game of life");
    // Setup game state -> game loop
    let mut state = MainState::new(ctx, config);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
