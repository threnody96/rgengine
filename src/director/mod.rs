mod application;
mod node;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use ::application::{ Application };
use ::util::{ must };
use ::node::{ Node, NodeLike, NodeDelegate, NodeId, SceneLike };
use self::application::ApplicationDirector;
use self::node::NodeDirector;
use sdl2::{ EventPump };
use sdl2::render::{ Canvas };
use sdl2::video::{ Window };

pub struct Director {
    canvas: RefCell<Option<Canvas<Window>>>,
    application: ApplicationDirector,
    node: NodeDirector
}

impl Director {

    pub fn new() -> Self {
        Self {
            canvas: RefCell::new(None),
            application: ApplicationDirector::new(),
            node: NodeDirector::new()
        }
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.application.get_scene()
    }

    pub fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        if self.application.get_scene().id() != scene.id() {
            let id = self.application.get_scene().id();
            self.destroy_node(&id);
        }
        self.application.set_scene(scene);
    }

    pub fn set_canvas(&self, canvas: Canvas<Window>) {
        self.canvas.replace(Some(canvas));
    }

    pub fn with_canvas<T, R>(&self, callback: T) -> R where T: FnOnce(&mut Canvas<Window>) -> R {
        let mut canvas = self.canvas.borrow_mut();
        if canvas.is_none() { must::<String, &str>(Err("ゲームが事項されていません")); }
        callback(canvas.as_mut().unwrap())
    }

    pub fn run_with_scene(&self, application: Rc<dyn Application>, scene: Rc<dyn SceneLike>) {
        self.application.set_application(application);
        self.application.set_scene(scene);
        let (mut event_pump, canvas) = self.application.build();
        self.set_canvas(canvas);
        self.application.run(&mut event_pump);
    }

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.node.register_node(node);
    }

    pub fn get_node<T>(&self, id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        self.node.get_node(id)
    }

    pub fn get_nodelike(&self, id: &NodeId) -> Option<Rc<dyn NodeLike>> {
        self.node.get_nodelike(id)
    }

    pub fn update_node(&self, id: &NodeId) {
        self.node.update(id);
    }

    pub fn render_node(&self, id: &NodeId) {
        self.node.render(id);
    }

    pub fn destroy_node(&self, id: &NodeId) {
        self.node.destroy(id);
    }

}
