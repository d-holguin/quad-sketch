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
            // No valid moves available, handle as needed.
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
