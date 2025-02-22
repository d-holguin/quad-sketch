use macroquad::prelude::*;
use quad_sketch_core::config;

const UI_OFFSET: f64 = 200.0;

pub struct Cell {
    alive: bool,
}

#[macroquad::main(config)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let grid = Grid::new(50, 50);

    let mut cells: Vec<Cell> = Vec::with_capacity((grid.rows * grid.cols) as usize);

    for cell in cells {

    }

    loop {
        clear_background(RED);
        grid.draw();



        for i in 0..=49 {
            for j in 0..=49 {
                grid.fill_cell(i, j);
            }
        }


        next_frame().await;
    }
}

struct Grid {
    rows: i32,
    cols: i32,
    cell_size_x: f32,
    cell_size_y: f32,
}


impl Grid {
    const LINE_WIDTH: f32 = 1.0;

    fn new(rows: i32, cols: i32) -> Self {
        let screen_width = screen_width() - Self::LINE_WIDTH;
        let screen_height = screen_height() - UI_OFFSET as f32 - Self::LINE_WIDTH;

        let cell_size_x = screen_width / cols as f32;
        let cell_size_y = screen_height / rows as f32;

        Self {
            rows,
            cols,
            cell_size_x,
            cell_size_y,
        }
    }

    fn draw(&self) {
        // Draw vertical lines
        let max_x = self.cols as f32 * self.cell_size_x + Grid::LINE_WIDTH;
        let max_y = self.rows as f32 * self.cell_size_y + Grid::LINE_WIDTH;

        for x in 0..=self.cols {
            let x_pos = (x as f32 * self.cell_size_x) + (Grid::LINE_WIDTH * 0.5);

            draw_line(
                x_pos,
                0.0,
                x_pos,
                max_y,
                Grid::LINE_WIDTH,
                LIGHTGRAY,
            );
        }

        // Draw horizontal lines
        for y in 0..=self.rows {
            let y_pos = (y as f32 * self.cell_size_y) + (Grid::LINE_WIDTH * 0.5);
            draw_line(
                0.0,
                y_pos,
                max_x,
                y_pos,
                Grid::LINE_WIDTH,
                LIGHTGRAY,
            );
        }
    }

    fn grid_to_screen(&self, grid_x: f32, grid_y: f32) -> Vec2 {
        // Convert grid coordinates to screen coordinates (uses precomputed values)
        vec2(grid_x * self.cell_size_x, grid_y * self.cell_size_y)
    }

    /// Fills a cell at the specified grid coordinates.
    fn fill_cell(&self, grid_x: i32, grid_y: i32) {
        let screen_coords = self.grid_to_screen(grid_x as f32, grid_y as f32);

        draw_rectangle(
            screen_coords.x + Grid::LINE_WIDTH,
            screen_coords.y + Grid::LINE_WIDTH,
            self.cell_size_x - Grid::LINE_WIDTH,
            self.cell_size_y - Grid::LINE_WIDTH,
            BLACK,
        );
    }
}
