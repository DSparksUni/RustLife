use raylib::prelude::*;

mod board;
mod render;

const BOARD_ROWS: i32 = 25;
const BOARD_COLS: i32 = 25;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const SIM_WIDTH: i32 = WINDOW_WIDTH - 200;
const SIM_HEIGHT: i32 = WINDOW_HEIGHT;

const SIM_INTERVALS: [f32; 5] = [0.1, 0.25, 0.5, 1.0, 2.0];

const UI_OFFSET: i32 = SIM_WIDTH;
const UI_WIDTH: i32 = WINDOW_WIDTH - SIM_WIDTH;
const UI_HEIGHT: i32 = WINDOW_HEIGHT;

const CLEAR_COLOR: Color = Color::new(160, 160, 160, 255);

const UI_BUTTON_WIDTH: f32 = 75.0;
const UI_BUTTON_HEIGHT: f32 = 25.0;

fn main() {
    let mut rl = render::RaylibContext::new("RustLife", WINDOW_WIDTH, WINDOW_HEIGHT);
    rl.handle.set_target_fps(60);

    let mut game_board = board::Board::new(BOARD_ROWS, BOARD_COLS);

    let mut sim_timer = 0.0;
    let mut sim_interval_index: i32 = 3;

    let mut pause = true;
    let mut pause_text = "Pause";

    let mut editing = true;
    let mut edit_text = "Stop Editing";

    while !rl.handle.window_should_close() {
        let speed_up_state;
        let slow_down_state;
        let pause_state;
        let edit_state;
        {
            // Put drawing in its own scope because DrawContext borrows the raylib handle
            let mut draw = rl.handle.begin_drawing(&rl.thread);

            draw.clear_background(CLEAR_COLOR);
            render::draw_board(&mut draw, &game_board, SIM_WIDTH, SIM_HEIGHT);

            // Speed Up button
            speed_up_state = draw.gui_button(
                Rectangle::new(
                    ((UI_OFFSET + UI_WIDTH / 2) as f32) - (UI_BUTTON_WIDTH / 2.0),
                    ((UI_HEIGHT / 6) as f32) - (UI_BUTTON_HEIGHT / 2.0),
                    UI_BUTTON_WIDTH,
                    UI_BUTTON_HEIGHT,
                ),
                "Speed Up",
            );

            // Slow Down button
            slow_down_state = draw.gui_button(
                Rectangle::new(
                    ((UI_OFFSET + UI_WIDTH / 2) as f32) - (UI_BUTTON_WIDTH / 2.0),
                    ((UI_HEIGHT / 6) as f32) + (UI_BUTTON_HEIGHT * 2.0) - (UI_BUTTON_HEIGHT / 2.0),
                    UI_BUTTON_WIDTH,
                    UI_BUTTON_HEIGHT,
                ),
                "Slow down",
            );

            // Pause button
            pause_state = draw.gui_button(
                Rectangle::new(
                    ((UI_OFFSET + UI_WIDTH / 2) as f32) - (UI_BUTTON_WIDTH / 2.0),
                    ((UI_HEIGHT as f32) / 1.15)
                        - (UI_BUTTON_HEIGHT * 2.0)
                        - (UI_BUTTON_HEIGHT / 2.0),
                    UI_BUTTON_WIDTH,
                    UI_BUTTON_HEIGHT,
                ),
                pause_text,
            );

            // Edit button
            edit_state = draw.gui_button(
                Rectangle::new(
                    ((UI_OFFSET + UI_WIDTH / 2) as f32) - (UI_BUTTON_WIDTH / 2.0),
                    ((UI_HEIGHT as f32) / 1.15) - (UI_BUTTON_HEIGHT / 2.0),
                    UI_BUTTON_WIDTH,
                    UI_BUTTON_HEIGHT,
                ),
                edit_text,
            );
        }

        if slow_down_state {
            // Slow Down button was pressed

            if sim_interval_index + 1 < SIM_INTERVALS.len() as i32 {
                sim_interval_index += 1;
            }
        } else if speed_up_state {
            // Speed Up button was pressed

            if sim_interval_index - 1 >= 0 {
                sim_interval_index -= 1;
            }
        } else if pause_state {
            // Pause button was pressed

            if !editing {
                pause = !pause;

                if pause {
                    pause_text = "Resume";
                } else {
                    pause_text = "Pause";
                }
            }
        } else if edit_state {
            // Edit button was pressed

            editing = !editing;
            pause = editing;

            if editing {
                edit_text = "Stop Editing";
            } else {
                edit_text = "Edit";
            }
        }

        if !pause {
            sim_timer += rl.handle.get_frame_time();
            if sim_timer >= SIM_INTERVALS[sim_interval_index as usize] {
                game_board.iterate();
                sim_timer = 0.0;
            }
        } else if editing {
            // Edit the board with the mouse
            let mouse_pos = rl.handle.get_mouse_position();

            if mouse_pos.x < SIM_WIDTH as f32 && mouse_pos.y < SIM_HEIGHT as f32 {
                let cell_col = (mouse_pos.x / (SIM_WIDTH / game_board.cols) as f32) as i32;
                let cell_row = (mouse_pos.y / (SIM_HEIGHT / game_board.rows) as f32) as i32;

                if rl
                    .handle
                    .is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    game_board.set_cell(cell_row, cell_col, true);
                } else if rl
                    .handle
                    .is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
                {
                    game_board.set_cell(cell_row, cell_col, false);
                }
            }
        }
    }
}
