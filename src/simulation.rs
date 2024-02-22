use orbclient::Color;

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
