use crate::scenes::{MainMenuScene, Scene, SceneChange};

use raylib::prelude::*;

pub struct GameplayScene {}

impl GameplayScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for GameplayScene {
    fn init(&mut self) {}

    fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) -> SceneChange {
        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            println!("Enter pressed");
            return SceneChange::Change(Box::new(MainMenuScene::new()));
        }
        SceneChange::None
    }

    fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_text("Gameplay Scene", 12, 12, 20, Color::BLACK);
    }

    fn destroy(&mut self) {}
}
