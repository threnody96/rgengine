use std::rc::Rc;
use std::collections::HashMap;
use ::application::{ Application };
use ::node::scene::{ SceneLike };
use ::node::scene::transition::{ SceneTransition, TransitionNone };
use ::node::label::{ LabelOption };
use ::util::parameter::{ Size };
use rand::{ Rng };
use rand::rngs::{ ThreadRng };
use rand::distributions::{ Standard, Distribution };

pub struct ApplicationDirector {
    scenes: Vec<Rc<dyn SceneLike>>,
    prev_scene: Option<Rc<dyn SceneLike>>,
    scene_transition: Rc<SceneTransition>,
    application: Option<Rc<dyn Application>>,
    label_option_aliases: HashMap<String, LabelOption>,
    current_fps: usize,
    rand: ThreadRng,
    continuing: bool
}

impl ApplicationDirector {

    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            prev_scene: None,
            scene_transition: TransitionNone::create(),
            application: None,
            label_option_aliases: HashMap::new(),
            current_fps: 0,
            rand: rand::thread_rng(),
            continuing: true
        }
    }

    pub fn window_size(&self) -> Size {
        self.application().window_size()
    }

    pub fn get_resolution_size(&self) -> Size {
        self.application().resolution_size()
    }

    pub fn rand<T>(&mut self) -> T where Standard: Distribution<T> {
        self.rand.gen()
    }

    pub fn is_continuing(&self) -> bool {
        self.continuing
    }

    pub fn set_continuing(&mut self, continuing: bool) {
        self.continuing = continuing;
    }

    pub fn default_label_option(&self) -> Option<LabelOption> {
        self.label_option_aliases.get("").cloned()
    }

    pub fn add_label_option_alias(&mut self, name: &str, option: &LabelOption, default: bool) {
        if name == "" { panic!("invalid label alias name"); }
        self.label_option_aliases.insert(name.to_owned(), option.clone());
        if default {
            self.label_option_aliases.insert("".to_owned(), option.clone());
        }
    }

    pub fn get_label_option(&self, name: &str) -> Option<LabelOption> {
        if name == "" { panic!("invalid label alias name"); }
        self.label_option_aliases.get(name).cloned()
    }

    pub fn replace_scene(&mut self, scene: Rc<dyn SceneLike>, transition: Rc<SceneTransition>) {
        if let Some(prev_scene) = self.scenes.pop() {
            self.destroy_prev_scene();
            self.prev_scene = Some(prev_scene);
        }
        self.scenes.push(scene);
        self.scene_transition = transition;
    }

    pub fn push_scene(&mut self, scene: Rc<dyn SceneLike>, transition: Rc<SceneTransition>) {
        if let Some(prev_scene) = self.scenes.last().cloned() {
            self.destroy_prev_scene();
            self.prev_scene = Some(prev_scene.clone());
        }
        self.scenes.push(scene);
        self.scene_transition = transition;
    }

    pub fn pop_scene(&mut self, transition: Rc<SceneTransition>) {
        if let Some(prev_scene) = self.scenes.pop() {
            self.destroy_prev_scene();
            self.prev_scene = Some(prev_scene);
        }
        self.scene_transition = transition;
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.scenes.last().cloned().unwrap()
    }

    pub fn get_prev_scene(&self) -> Option<Rc<dyn SceneLike>> {
        self.prev_scene.clone()
    }

    pub fn get_scene_transition(&self) -> Rc<SceneTransition> {
        self.scene_transition.clone()
    }

    pub fn destroy_prev_scene(&mut self) {
        if let Some(prev_scene) = self.prev_scene.clone() {
            let mut popable = false;
            let id = prev_scene.id();
            for scene in &self.scenes {
                if scene.id() == id {
                    popable = true;
                    break;
                }
            }
            if !popable { prev_scene.destroy(); }
            self.prev_scene = None;
        }
    }

    pub fn set_application(&mut self, application: Rc<dyn Application>) {
        self.application = Some(application);
    }

    pub fn application(&self) -> Rc<dyn Application> {
        self.application.clone().ok_or("application not found").unwrap()
    }

    pub fn set_current_fps(&mut self, current_fps: usize) {
        self.current_fps = current_fps;
    }

    pub fn current_fps(&self) -> usize {
        self.current_fps
    }

}
