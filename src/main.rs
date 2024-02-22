use orbclient::{Color, Renderer, Window};
use rand::Rng;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

/// Count the number of live neighbors for a given cell
fn count_live_neighbors(grid: &[Vec<bool>], row: usize, col: usize) -> u8 {
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
                && grid[i as usize][j as usize]
            {
                count += 1;
            }
        }
    }

    count
}

/// Update the game board based on the rules of Conway's Game of Life
fn update_board(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let live_neighbors = count_live_neighbors(grid, i, j);

            // Rules of Conway's Game of Life:
            // 1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
            // 2. Any live cell with two or three live neighbors lives on to the next generation.
            // 3. Any live cell with more than three live neighbors dies, as if by overpopulation.
            // 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

            match grid[i][j] {
                true => {
                    // Rules 1-3
                    new_grid[i][j] = (2..=3).contains(&live_neighbors);
                }
                false => {
                    // Rule 4
                    if live_neighbors == 3 {
                        new_grid[i][j] = true;
                    }
                }
            }
        }
    }

    *grid = new_grid;
}

fn main() {
    println!("Starting simulation...");

    // Open a window
    let (width, height) = orbclient::get_display_size().expect("Failed to get display size");
    let mut window = Window::new(
        (width as i32) / 4,
        (height as i32) / 4,
        width / 2,
        height / 2,
        "Conway's Game of Life",
    )
    .expect("Failed to open window");

    let (win_w, win_h) = (width / 8, height / 8);

    let mut grid = vec![vec![false; win_w as usize]; win_h as usize];
    let mut rng = rand::thread_rng();
    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            *cell = rng.gen_bool(0.5);
        });
    });

    // Create a flag that will be set to true when a quit signal is received
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Register a handler function that will be called when a quit signal is received
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let update_interval = Duration::from_millis(1000 / 30); // 30 FPS
    let mut last_update = Instant::now();

    loop {
        // Check if a quit signal has been received
        if !running.load(Ordering::SeqCst) {
            break;
        }

        // Limit the frame rate to 30 FPS
        if Instant::now() - last_update < update_interval {
            continue;
        }
        last_update = Instant::now();

        // Draw the game state to the window
        window.clear();
        for i in 0..win_h {
            for j in 0..win_w {
                if grid[i as usize][j as usize] {
                    window.rect(j as i32 * 4, i as i32 * 4, 4, 4, Color::rgb(255, 255, 255));
                }
            }
        }

        update_board(&mut grid);

        window.sync();
    }
}
