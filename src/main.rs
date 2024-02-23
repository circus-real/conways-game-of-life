use crate::simulation::SimulationState;
use orbclient::{Color, Renderer, Window};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

mod simulation;

const FRAME_RATE: u64 = 3;
const START_ALIVE_PROB: f64 = 0.3;

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

    let mut grid = SimulationState::new(win_h as usize, win_w as usize, START_ALIVE_PROB);

    // Create a flag that will be set to true when a quit signal is received
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    // Register a handler function that will be called when a quit signal is received
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let update_interval = Duration::from_secs(1 / FRAME_RATE);
    let mut last_update = Instant::now();

    loop {
        // Check if a quit signal has been received
        if !running.load(Ordering::SeqCst) {
            break;
        }

        // Limit the frame rate
        if Instant::now() - last_update < update_interval {
            continue;
        }
        last_update = Instant::now();

        // Draw the game state to the window
        window.clear();
        for i in 0..win_h {
            for j in 0..win_w {
                let cell_color = grid.get_cell(i as usize, j as usize).get_color();
                if cell_color == Color::rgb(0, 0, 0) {
                    continue;
                }
                window.rect(j as i32 * 4, i as i32 * 4, 4, 4, cell_color);
            }
        }

        grid.update();

        window.sync();
    }
}
