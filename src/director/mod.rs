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
use ::resource::{ RTexture, RFont };
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

    pub fn set_application(&self, application: Rc<dyn Application>) {
        let mut app = self.application.borrow_mut();
        app.set_application(application);
    }

    pub fn set_scene_first(&self, scene: Rc<dyn SceneLike>) {
        self.application.borrow_mut().set_scene(scene);
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

    pub fn load_plain_data(&self, path: &str) -> Rc<Vec<u8>> {
        self.resource.borrow_mut().load_plain_data(path)
    }

    pub fn load_texture(&self, path: &str) -> RTexture {
        self.resource.borrow_mut().load_texture(path)
    }

    pub fn load_font(&self, path: &str, point: u16) -> RFont {
        self.resource.borrow_mut().load_font(path, point)
    }

}
