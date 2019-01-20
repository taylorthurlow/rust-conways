extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const BOARD_SIZE: usize = 100;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (600, 600);

    let mut window: PistonWindow = WindowSettings::new("Conway's", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];

    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            let randi = rand::thread_rng().gen_range(1, 101);
            *cell = randi < 50;
        }
    }

    let mut counter = 1;

    while let Some(event) = window.next() {
        println!("{}", counter);
        draw_board(&mut window, event, board);
        board = tick(board);
        counter += 1;
    }
}

fn tick(board: [[bool; BOARD_SIZE]; BOARD_SIZE]) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    let mut new_board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let (alive, _) = neighbor_counts(board, x, y);

            new_board[y][x] = match *cell {
                true => {
                    // Live cell
                    match alive {
                        // Match number of alive neighbors
                        0..=1 => false,
                        2..=3 => true,
                        4..=8 => false,
                        _ => false,
                    }
                },
                false => {
                    // Dead cell
                    match alive {
                        // Match number of alive neighbors
                        3 => true,
                        _ => false,
                    }
                }
            };
        }
    }

    new_board
}

fn neighbor_counts(board: [[bool; BOARD_SIZE]; BOARD_SIZE], x: usize, y: usize) -> (usize, usize) {
    let mut total_alive = 0;
    let mut total_dead = 0;

    let diffs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in diffs.iter() {
        let new_x = x as isize + *dx;
        let new_y = y as isize + *dy;

        if new_x >= 0 && new_y >= 0 && new_x < BOARD_SIZE as isize && new_y < BOARD_SIZE as isize {
            if board[new_y as usize][new_x as usize] {
                total_alive += 1;
            } else {
                total_dead += 1;
            }
        }
    }

    (total_alive, total_dead)
}

fn draw_board(
    window: &mut PistonWindow,
    event: piston_window::Event,
    board: [[bool; BOARD_SIZE]; BOARD_SIZE],
) {
    window.draw_2d(&event, |context, graphics| {
        clear([1.0; 4], graphics);

        // Draw cells
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                rectangle(
                    match cell {
                        true => [1.0, 1.0, 1.0, 1.0],  // white
                        false => [0.0, 0.0, 0.0, 1.0], // black
                    },
                    [x as f64 * 6.0, y as f64 * 6.0, 6.0, 6.0],
                    context.transform,
                    graphics,
                )
            }
        }
    });
}
