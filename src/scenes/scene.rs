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
    next_scene: Option<Box<dyn Scene>>,
    on_transition: bool,
    trans_fadeout: bool,
    trans_alpha: f32,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: Box::new(MainMenuScene::new()),
            next_scene: None,
            on_transition: false,
            trans_fadeout: false,
            trans_alpha: 0.0,
        }
    }

    fn start_transition(&mut self) {
        self.on_transition = true;
        self.trans_fadeout = true;
        self.trans_alpha = 0.0;
    }

    fn update_transition(&mut self) {
        if self.trans_fadeout {
            // fade out
            self.trans_alpha += 0.05;
            if self.trans_alpha > 1.01 {
                self.trans_alpha = 1.0;
                self.trans_fadeout = false; // switch to fade in

                // perform the change scene now that screen is black
                if let Some(next_scene) = self.next_scene.take() {
                    self.current_scene.destroy();
                    self.current_scene = next_scene;
                    self.current_scene.init();
                }
            }
        } else {
            // fade in
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
                // Store the new scene and start the fade-out
                self.next_scene = Some(new_scene);
                self.start_transition();
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
