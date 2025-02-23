use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use quad_sketch_core::config;

const UI_OFFSET: f64 = 200.0;
const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;
const UPDATES_PER_SECOND: f64 = 10.0;
const TIME_STEP: f64 = 1.0 / UPDATES_PER_SECOND;

#[derive(Clone)]
pub struct Cell {
    alive: bool,
}

#[macroquad::main(config)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let grid = Grid::new(50, 50);

    let mut cells = create_cells(grid.rows, grid.cols);
    let mut next_cells = cells.clone();

    let mut last_update_time = get_time();

    let screen_h = screen_height();
    let screen_w = screen_width();

    loop {
        clear_background(BLACK);
        grid.draw();

        for row in 0..grid.rows {
            for col in 0..grid.cols {
                let idx = (row * grid.cols) + col;
                if cells[idx].alive {
                    grid.fill_cell(row as i32, col as i32);
                }
            }
        }

        let current_time = get_time();
        if current_time - last_update_time > TIME_STEP {
            update_cells(&mut cells, &mut next_cells, grid.rows, grid.cols);
            last_update_time = current_time;
        }

        let ui_y = screen_h - UI_OFFSET as f32;
        root_ui().window(hash!(), vec2(0.0, ui_y), vec2(screen_w, UI_OFFSET as f32), |ui| {
            ui.label(vec2(screen_w / 2.0, 5.0), "Game of Life");

            if ui.button(Some(vec2(20.0, 40.0)), "Reset") {
                reset_cells(&mut cells, &mut next_cells, grid.rows, grid.cols);
            }
        });




        next_frame().await;
    }
}

fn reset_cells(cells: &mut Vec<Cell>, next_cells: &mut Vec<Cell>, rows: usize, cols: usize) {
    let new_cells = create_cells(rows, cols);
    *cells = new_cells.clone();
    *next_cells = new_cells;
}

struct Grid {
    rows: usize,
    cols: usize,
    cell_size_x: f32,
    cell_size_y: f32,
}


impl Grid {
    const LINE_WIDTH: f32 = 1.0;
    const GRID_COLOR: Color = WHITE;

    const CELL_COLOR: Color = GREEN;


    fn new(rows: usize, cols: usize) -> Self {
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
                Grid::GRID_COLOR,
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
                Grid::GRID_COLOR,
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
            Self::CELL_COLOR,
        );
    }
}


fn create_cells(rows: usize, cols: usize) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(rows * cols);

    for _ in 0..(rows * cols) {
        let alive: bool = rand::gen_range(0, 2) == 0;
        cells.push(Cell { alive });
    }

    cells
}

fn count_alive_neighbors(cells: &Vec<Cell>, row: usize, col: usize, rows: usize, cols: usize) -> usize {
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /* (this cell) */ (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    offsets.iter().filter(|&&(dr, dc)| {
        let nr = row.wrapping_add(dr as usize);
        let nc = col.wrapping_add(dc as usize);
        nr < rows && nc < cols && cells[(nr * cols) + nc].alive
    }).count()
}

/// A live cell dies if it has fewer than two live neighbors.
/// A live cell with two or three live neighbors lives on to the next generation.
/// A live cell with more than three live neighbors dies.
/// A dead cell will be brought back to live if it has exactly three live neighbors.
fn update_cells(cells: &mut Vec<Cell>, next_cells: &mut Vec<Cell>, rows: usize, cols: usize) {
    for row in 0..rows {
        for col in 0..cols {
            let alive_neighbors = count_alive_neighbors(cells, row, col, rows, cols);
            let idx = (row * cols) + col;

            next_cells[idx].alive = match (cells[idx].alive, alive_neighbors) {
                (true, 2) | (true, 3) => true, // survive
                (false, 3) => true, // born
                _ => false, // dies
            }
        }
    }
    std::mem::swap(cells, next_cells);
}