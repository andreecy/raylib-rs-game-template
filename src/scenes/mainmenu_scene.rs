use crate::scenes::{GameplayScene, Scene, SceneChange};
use raylib::prelude::*;

pub struct MainMenuScene {}

impl MainMenuScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for MainMenuScene {
    fn init(&mut self) {}

    fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) -> SceneChange {
        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            println!("Enter pressed");
            return SceneChange::Change(Box::new(GameplayScene::new()));
        }
        SceneChange::None
    }

    fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text("Main Menu Scene", 12, 12, 20, Color::GREEN);
    }

    fn destroy(&mut self) {}
}
