use crate::scenes::{GameplayScene, Scene};
use raylib::prelude::*;

pub struct MainMenuScene {
    next_scene: Option<Box<dyn Scene>>,
}

impl MainMenuScene {
    pub fn new() -> Self {
        Self { next_scene: None }
    }
}

impl Scene for MainMenuScene {
    fn init(&mut self) {}

    fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            println!("Enter pressed");
            self.next_scene = Some(Box::new(GameplayScene::new()));
        }
    }

    fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text("Main Menu Scene", 12, 12, 20, Color::GREEN);
    }

    fn destroy(&mut self) {}

    fn get_next_scene(&mut self) -> Option<Box<dyn Scene>> {
        if self.next_scene.is_none() {
            None
        } else {
            Some(self.next_scene.take().unwrap())
        }
    }
}
