use game_of_life_core::config;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use std::cmp;

const UI_OFFSET: f64 = 100.0;


#[macroquad::main(config)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;

    let mut grid_size = 100.0;

    let mut grid = Grid::new(grid_size as usize);

    let mut cells = create_cells(grid.rows, grid.cols);
    let mut next_cells = cells.clone();

    let mut last_update_time = get_time();

    let screen_h = screen_height();
    let screen_w = screen_width();

    let mut updates_per_second: f32 = 10.0;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::MainMenu => {
                let ui_w = screen_w * 0.25;
                let ui_h = screen_h * 0.25;
                root_ui().window(
                    hash!(),
                    vec2(
                        (screen_w / 2.0) - (ui_w / 2.0),
                        (screen_h / 2.0) - (ui_h / 2.0),
                    ),
                    vec2(ui_w, ui_h),
                    |ui| {

                        widgets::Slider::new(hash!(), 10.0..650.0)
                            .label("Grid Size")
                            .ui(ui, &mut grid_size);
                        grid_size = grid_size.round();
                        let grid_size = grid_size.round();

                        if ui.button(Some(vec2(ui_w / 2.0, 30.0)), "Start") {
                            grid = Grid::new(grid_size as usize);
                            cells = create_cells(grid.rows, grid.cols);
                            next_cells = cells.clone();
                            game_state = GameState::Running;
                        }
                    },
                );
            }
            GameState::Running => {
                let time_step: f64 = 1.0 / updates_per_second as f64;

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
                if current_time - last_update_time > time_step {
                    update_cells(&mut cells, &mut next_cells, grid.rows, grid.cols);
                    last_update_time = current_time;
                }

                root_ui().window(
                    hash!(),
                    vec2(0.0, screen_h - UI_OFFSET as f32),
                    vec2(screen_w, UI_OFFSET as f32),
                    |ui| {
                        ui.slider(hash!(), "Updates", 1.0..30.0, &mut updates_per_second);
                        updates_per_second = updates_per_second.round();

                        ui.separator();

                        if ui.button(vec2(5.0, 30.0), "Reset") {
                            reset_cells(&mut cells, &mut next_cells, grid.rows, grid.cols);
                        }

                        if ui.button(vec2(5.0, 60.0), "Menu") {
                            game_state = GameState::MainMenu;
                        }
                        let fps_text = format!("FPS: {}", get_fps());
                        ui.label(vec2(screen_w - 100.0, 30.0), fps_text.as_str());
                    },
                );
            }
        }
        next_frame().await;
    }
}

#[derive(Clone)]
pub struct Cell {
    alive: bool,
}

enum GameState {
    MainMenu,
    Running,
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
    line_thickness: f32,
}

impl Grid {
    const GRID_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.2);

    const CELL_COLOR: Color = GREEN;

    fn new(rows: usize) -> Self {
        let cols = rows;

        let line_thickness = Grid::calculate_line_thickness(rows, cols);

        let screen_width = screen_width() - line_thickness;
        let screen_height = screen_height() - UI_OFFSET as f32 - line_thickness;

        let cell_size_x = screen_width / cols as f32;
        let cell_size_y = screen_height / rows as f32;

        Self {
            rows,
            cols,
            cell_size_x,
            cell_size_y,
            line_thickness,
        }
    }

    fn calculate_line_thickness(rows: usize, cols: usize) -> f32 {
        let grid_size = cmp::max(rows, cols) as f32;

        if grid_size > 150.0 {
            return 0.0
        }

        let min_thickness = 1.0;
        let max_thickness = 3.0;

        let min_grid = 5.0;
        let max_grid = 150.0;



        let t = ((grid_size - min_grid) / (max_grid - min_grid)).clamp(0.0, 1.0);

        max_thickness + (min_thickness - max_thickness) * t
    }

    fn draw(&self) {
        // Draw vertical lines
        let max_x = self.cols as f32 * self.cell_size_x + self.line_thickness;
        let max_y = self.rows as f32 * self.cell_size_y + self.line_thickness;

        for x in 0..=self.cols {
            let x_pos = (x as f32 * self.cell_size_x) + (self.line_thickness * 0.5);

            draw_line(
                x_pos,
                0.0,
                x_pos,
                max_y,
                self.line_thickness,
                Grid::GRID_COLOR,
            );
        }

        // Draw horizontal lines
        for y in 0..=self.rows {
            let y_pos = (y as f32 * self.cell_size_y) + (self.line_thickness * 0.5);
            draw_line(
                0.0,
                y_pos,
                max_x,
                y_pos,
                self.line_thickness,
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
            screen_coords.x + self.line_thickness,
            screen_coords.y + self.line_thickness,
            self.cell_size_x - self.line_thickness,
            self.cell_size_y - self.line_thickness,
            Self::CELL_COLOR,
        );
    }
}

fn create_cells(rows: usize, cols: usize) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(rows * cols);

    for _ in 0..(rows * cols) {
        let chance = rand::gen_range(0, 9);

        if chance < 2 {
            cells.push(Cell { alive: true });
        } else {
            cells.push(Cell { alive: false });
        }
    }

    cells
}

fn count_alive_neighbors(
    cells: &Vec<Cell>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        /* (this cell) */ (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    offsets
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = (row as isize + dr + rows as isize) as usize % rows;
            let nc = (col as isize + dc + cols as isize) as usize % cols;
            cells[(nr * cols) + nc].alive
        })
        .count()
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
                (false, 3) => true,            // born
                _ => false,                    // dies
            }
        }
    }
    std::mem::swap(cells, next_cells);
}
