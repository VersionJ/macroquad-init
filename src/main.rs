use macroquad::prelude::*;

#[macroquad::main(window_config)]
async fn main() {
    // outside game loop
    loop {
        // do logic
        
        // clear background and draw
        clear_background(WHITE);

        // wait until next frame
        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Title".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        icon: None,
        platform: Default::default(),
    }
}