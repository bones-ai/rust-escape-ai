use escape::simulation::SimulationStats;
use macroquad::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use escape::editor::Editor;
use escape::resources::init_resources;
use escape::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Escape".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: IS_FULL_SCREEN,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    init_resources().await;

    let mut editor = Editor::new();
    let mut simulation = Simulation::new();
    let mut stats = SimulationStats::new();

    loop {
        let (r, g, b, a) = WINDOW_BACKGROUND_COLOR;
        clear_background(Color::from_rgba(r, g, b, a));

        if editor.settings.is_frame_skip {
            for _ in 0..10 {
                stats = simulation.update(&editor).unwrap_or(stats);
            }
        }

        stats = simulation.update(&editor).unwrap_or(stats);
        simulation.draw(&editor);

        editor.update();
        editor.draw(&stats);

        if editor.settings.slow_mode {
            sleep(Duration::from_millis(200));
        }
        if editor.settings.is_restart {
            editor.settings.is_restart = false;
            simulation = Simulation::new();
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
