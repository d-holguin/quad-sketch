use macroquad::prelude::*;

fn config() -> Conf {
    Conf {
        window_title: "Simple Game".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let grid = Grid::new(10, 10);
    loop {
        clear_background(WHITE);
        grid.draw();
        let point1 = grid.grid_to_screen(5.0, 5.0);
        let (mouse_x, mouse_y) = mouse_position();
        let point2 = vec2(mouse_x, mouse_y);
        draw_circle(point1.x, point1.y, 0.1 * grid.scale_x(), RED);
        draw_circle(point2.x, point2.y, 0.1 * grid.scale_x(), RED);
        draw_line(point1.x, point1.y, point2.x, point2.y, 1.0, RED);
        next_frame().await;
    }
}
struct Grid {
    rows: i32,
    cols: i32,
}
impl Grid {
    fn new(rows: i32, cols: i32) -> Self {
        Self {
            rows,
            cols,
        }
    }

    fn scale_x(&self) -> f32 {
        screen_width() / self.cols as f32
    }

    fn scale_y(&self) -> f32 {
        screen_height() / self.rows as f32
    }

    fn draw(&self) {
        let scale_x = self.scale_x();
        let scale_y = self.scale_y();

        // Draw vertical lines
        for x in 0..=self.cols {
            let x_pos = x as f32 * scale_x;
            draw_line(x_pos, 0.0, x_pos, screen_height(), 1.0, BLACK);
        }
        // Draw horizontal lines
        for y in 0..=self.rows {
            let y_pos = y as f32 * scale_y;
            draw_line(0.0, y_pos, screen_width(), y_pos, 1.0, BLACK);
        }
    }

    fn grid_to_screen(&self, grid_x: f32, grid_y: f32) -> Vec2 {
        vec2(grid_x * self.scale_x(), grid_y * self.scale_y())
    }
}

fn draw_grid(rows: i32, cols: i32) {
    let screen_height = screen_height();
    let screen_width = screen_width();
    let scale_x = screen_width / cols as f32;
    let scale_y = screen_height / rows as f32;
    for x in 0..=rows {
        let x_pos = x as f32 * scale_x;
        draw_line(x_pos, 0.0, x_pos, screen_height, 1.0, BLACK);
    }
    for y in 0..=cols {
        let y_pos = y as f32 * scale_y;
        draw_line(0.0, y_pos, screen_width, y_pos, 1.0, BLACK);
    }
    let center = vec2(5.0 * scale_x, 5.0 * scale_y);

    draw_circle(center.x, center.y, 0.5 * scale_x, RED);
}
pub struct Walker {
    x: f32,
    y: f32,
    stride: f32,
    previous_steps: Vec<(f32, f32)>,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            stride: 50.0,
            previous_steps: Vec::new(),
        }
    }
    pub fn step(&mut self) {
        let directions = vec![(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)];

        let valid_steps = directions
            .iter()
            .filter(|&&(dx, dy)| {
                let new_x = self.x + dx * self.stride;
                let new_y = self.y + dy * self.stride;
                new_x >= 0.0 && new_x < screen_width() && new_y >= 0.0 && new_y < screen_height()
            })
            .collect::<Vec<_>>();

        if valid_steps.is_empty() {
            println!("Walker is stuck, reinitializing position");
            self.x = screen_width() / 2.0;
            self.y = screen_height() / 2.0;
            self.previous_steps.clear();
        } else {
            let (dx, dy) = valid_steps[rand::gen_range(0, valid_steps.len())];
            self.x += dx * self.stride;
            self.y += dy * self.stride;
            self.previous_steps.push((self.x, self.y));
        }
    }
}


