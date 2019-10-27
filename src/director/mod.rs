mod application;
mod node;
mod resource;
mod render;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use ::application::{ Application };
use ::util::{ must };
use ::node::{ Node, NodeLike, NodeDelegate, NodeId, SceneLike };
use self::application::ApplicationDirector;
use self::node::NodeDirector;
use self::resource::ResourceDirector;
use sdl2::{ EventPump };
use sdl2::render::{ Canvas, Texture, TextureCreator };
use sdl2::video::{ Window, WindowContext };
pub use self::render::RenderDirector;

pub struct Director {
    application: RefCell<ApplicationDirector>,
    node: RefCell<NodeDirector>,
    resource: RefCell<ResourceDirector>
}

impl Director {

    pub fn new() -> Self {
        Self {
            application: RefCell::new(ApplicationDirector::new()),
            node: RefCell::new(NodeDirector::new()),
            resource: RefCell::new(ResourceDirector::new())
        }
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.application.borrow().get_scene()
    }

    pub fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        let current_scene = { self.application.borrow().get_scene() };
        if current_scene.id() != scene.id() {
            let id = current_scene.id();
            self.destroy_node(&id);
        }
        self.application.borrow_mut().set_scene(scene);
    }

    pub fn title(&self) -> String {
        self.application.borrow().title()
    }

    pub fn init(&self, application: Rc<dyn Application>, scene: Rc<dyn SceneLike>) {
        let mut app = self.application.borrow_mut();
        app.set_application(application);
        app.set_scene(scene);
    }

    pub fn fps(&self) -> u32 {
        self.application.borrow().fps()
    }

    pub fn generate_id(&self) -> String {
        self.application.borrow_mut().generate_id()
    }

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.node.borrow_mut().register_node(node);
    }

    pub fn get_node<T>(&self, id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        self.node.borrow().get_node(id)
    }

    pub fn get_nodelike(&self, id: &NodeId) -> Option<Rc<dyn NodeLike>> {
        self.node.borrow().get_nodelike(id)
    }

    pub fn destroy_node(&self, id: &NodeId) {
        self.node.borrow_mut().destroy(id);
    }

}
