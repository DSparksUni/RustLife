use crate::board::Board;
use raylib::{drawing::RaylibDrawHandle, prelude::*};

pub struct RaylibContext {
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
}
impl RaylibContext {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let (rl, t) = raylib::init().size(width, height).title(title).build();

        Self {
            handle: rl,
            thread: t,
        }
    }
}

pub fn draw_board(
    draw: &mut RaylibDrawHandle,
    game_board: &Board,
    render_width: i32,
    render_height: i32,
) {
    let cell_width = render_width / game_board.cols;
    let cell_height = render_height / game_board.rows;

    for row in 0..game_board.rows {
        for col in 0..game_board.cols {
            let cell_color;
            if game_board.get_cell(row, col) {
                cell_color = Color::GRAY;
            } else {
                cell_color = Color::WHITE;
            }

            draw.draw_rectangle(
                col * cell_width,
                row * cell_height,
                cell_width,
                cell_height,
                cell_color,
            );
            draw.draw_rectangle_lines(
                col * cell_width,
                row * cell_height,
                cell_width,
                cell_height,
                Color::BLACK,
            );
        }
    }
}
