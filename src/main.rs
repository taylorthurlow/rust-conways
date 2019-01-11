extern crate piston_window;
extern crate rand;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (600, 600);

    let mut window: PistonWindow = WindowSettings::new("Conway's", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    const BOARD_SIZE: usize = 200;
    let mut board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];

    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = rand::random();
        }
    }

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            // Draw background
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black
                [0.0, 0.0, 300.0, 300.0],
                context.transform,
                graphics,
            );

            // Draw cells
            for (y, row) in board.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    rectangle(
                        match cell {
                            true => [1.0, 1.0, 1.0, 1.0], // white
                            false => [0.0, 0.0, 0.0, 1.0],  // black
                        },
                        [x as f64 * 3.0, y as f64 * 3.0, 3.0, 3.0],
                        context.transform,
                        graphics,
                    )
                }
            }
        });
    }
}
