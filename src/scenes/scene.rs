use super::mainmenu_scene::MainMenuScene;
use raylib::prelude::*;

pub enum SceneChange {
    None,
    Change(Box<dyn Scene>),
}

pub trait Scene {
    fn init(&mut self) {}

    fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) -> SceneChange;

    fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle);

    fn destroy(&mut self) {}
}

pub struct SceneManager {
    current_scene: Box<dyn Scene>,
    on_transition: bool,
    trans_fadeout: bool,
    trans_alpha: f32,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: Box::new(MainMenuScene::new()),
            on_transition: false,
            trans_fadeout: false,
            trans_alpha: 0.0,
        }
    }

    fn start_transition(&mut self) {
        self.on_transition = true;
        self.trans_fadeout = false;
        self.trans_alpha = 0.0;
    }

    fn update_transition(&mut self) {
        if !self.trans_fadeout {
            self.trans_alpha += 0.05;
            if self.trans_alpha > 1.01 {
                self.trans_alpha = 1.0;
                self.trans_fadeout = true;
            }
        } else {
            self.trans_alpha -= 0.05;
            if self.trans_alpha < -0.01 {
                self.trans_alpha = 0.0;
                self.on_transition = false;
            }
        }
    }

    pub fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        if !self.on_transition {
            let change_request = self.current_scene.update(d);
            if let SceneChange::Change(new_scene) = change_request {
                self.start_transition();
                self.current_scene.destroy();
                self.current_scene = new_scene;
                self.current_scene.init();
            }
        } else {
            self.update_transition();
        }
    }

    fn draw_transition(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::BLACK.alpha(self.trans_alpha),
        );
    }

    pub fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        self.current_scene.draw(d);

        if self.on_transition {
            self.draw_transition(d);
        }
    }
}
