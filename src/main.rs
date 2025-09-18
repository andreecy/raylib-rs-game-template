use raylib::prelude::*;

mod scenes;
use scenes::SceneManager;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut scene_manager = SceneManager::new();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        scene_manager.update(&mut d);

        d.clear_background(Color::BLACK);
        scene_manager.draw(&mut d);
    }
}
