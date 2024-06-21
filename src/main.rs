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
    let mut walker = Walker::new();
    let walker_size = walker.stride;
    loop {
        clear_background(WHITE);

        walker.step();
        for step in &walker.previous_steps {
            draw_rectangle(step.0, step.1, walker_size, walker_size, BLACK)
        }

        next_frame().await;
    }
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
            stride: 1.0,
            previous_steps: Vec::new(),
        }
    }
    pub fn step(&mut self) {
        loop {
            let x_step = rand::gen_range(0, 3) - 1;
            let y_step = rand::gen_range(0, 3) - 1;

            let new_x = self.x + x_step as f32 * self.stride;
            let new_y = self.y + y_step as f32 * self.stride;

            if new_x < 0.0
                || new_x > screen_width()
                || new_y < 0.0
                || new_y > screen_height()
                || self.previous_steps.contains(&(new_x, new_y))
            {
                continue;
            }
            self.x = new_x;
            self.y = new_y;
            self.previous_steps.push((self.x, self.y));
            break;
        }
    }
}

fn _create_checkerboard() {
    let rows = 100;
    let cols = 100;

    let cell_width = screen_width() / cols as f32;
    let cell_height = screen_height() / rows as f32;

    for i in 0..rows {
        for j in 0..cols {
            let x = j as f32 * cell_width;
            let y = i as f32 * cell_height;
            let color = if (i + j) % 2 == 0 { BLACK } else { WHITE };
            draw_rectangle(x, y, cell_width, cell_height, color);
        }
    }
}
