use orbclient::Color;
use rand::Rng;

/// The state of a cell in the game grid
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CellState {
    #[default]
    Dead,
    Alive,
}

impl CellState {
    /// Get the color of a cell based on its state
    pub fn get_color(&self) -> Color {
        match self {
            CellState::Alive => Color::rgb(255, 255, 255),
            CellState::Dead => Color::rgb(0, 0, 0),
        }
    }
    /// Count the number of live neighbors for a given cell
    fn count_live_neighbors(&self, grid: &[Vec<CellState>], row: usize, col: usize) -> u8 {
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for i in (row as i32 - 1)..=(row as i32 + 1) {
            for j in (col as i32 - 1)..=(col as i32 + 1) {
                if i >= 0
                    && i < rows as i32
                    && j >= 0
                    && j < cols as i32
                    && !(i == row as i32 && j == col as i32)
                    && grid[i as usize][j as usize] == CellState::Alive
                {
                    count += 1;
                }
            }
        }

        count
    }

    /// Get the next state of a cell based on the rules of Conway's Game of Life
    pub fn next_state(&self, grid: &[Vec<CellState>], row: usize, col: usize) -> CellState {
        let live_neighbors = self.count_live_neighbors(grid, row, col);
        match self {
            CellState::Alive => {
                if !(2..=3).contains(&live_neighbors) {
                    CellState::Dead
                } else {
                    CellState::Alive
                }
            }
            CellState::Dead => {
                if live_neighbors == 3 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            }
        }
    }
}

/// The simulation state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimulationState {
    pub grid: Vec<Vec<CellState>>,
    pub rows: usize,
    pub cols: usize,
}

impl SimulationState {
    /// Create a new simulation state
    pub fn new(rows: usize, cols: usize, start_alive_prob: f64) -> Self {
        let mut grid = vec![vec![CellState::default(); cols]; rows];
        let mut rng = rand::thread_rng();
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                let alive = rng.gen_bool(start_alive_prob);
                *cell = if alive {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            });
        });
        Self { grid, rows, cols }
    }

    /// Get the next state of the simulation
    pub fn update(&mut self) {
        let prev_grid = self.grid.clone();
        self.grid.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, cell)| {
                *cell = cell.next_state(&prev_grid, i, j);
            });
        });
    }

    /// Get the contents of a cell at a given position
    pub fn get_cell(&self, row: usize, col: usize) -> CellState {
        self.grid[row][col]
    }
}
