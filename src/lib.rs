use macroquad::window::Conf;

pub fn config() -> Conf {
    let aspect_ratio = 16.0 / 9.0; // = ~1.7
    let width = 1800.0;
    let height = width / aspect_ratio;

    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: width as i32,
        window_height: height as i32,
        window_resizable: false,
        ..Default::default()
    }
}