use super::mainmenu_scene::MainMenuScene;

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
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: Box::new(MainMenuScene::new()),
        }
    }

    pub fn update(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        let change_request = self.current_scene.update(d);
        if let SceneChange::Change(new_scene) = change_request {
            self.current_scene.destroy();
            self.current_scene = new_scene;
            self.current_scene.init();
        }
    }

    pub fn draw(&mut self, d: &mut raylib::core::drawing::RaylibDrawHandle) {
        self.current_scene.draw(d);
    }
}
