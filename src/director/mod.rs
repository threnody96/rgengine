mod application;
mod node;
mod resource;
mod render;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use ::application::{ Application };
use ::util::{ Size, Point, Must };
use ::node::{ Node, NodeLike, NodeDelegate, NodeId, SceneLike, LabelOption };
use ::resource::{ RTexture, RFont };
use self::application::ApplicationDirector;
use self::node::NodeDirector;
use self::resource::ResourceDirector;
use sdl2::{ EventPump };
use sdl2::render::{ Canvas, Texture, TextureCreator };
use sdl2::video::{ Window, WindowContext };
use sdl2::ttf::FontStyle;
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

    pub fn window_size(&self) -> Size {
        self.application.borrow().window_size()
    }

    pub fn is_continuing(&self) -> bool {
        self.application.borrow().is_continuing()
    }

    pub fn set_continuing(&self, continuing: bool) {
        self.application.borrow_mut().set_continuing(continuing);
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.application.borrow().get_scene()
    }

    pub fn replace_scene(&self, scene: Rc<dyn SceneLike>) {
        let current_scene = { self.application.borrow().get_scene() };
        if current_scene.id() != scene.id() {
            let id = current_scene.id();
            self.destroy_node(&id);
        }
        self.application.borrow_mut().set_scene(scene);
    }

    pub fn set_application(&self, application: Rc<dyn Application>) {
        let mut app = self.application.borrow_mut();
        app.set_application(application);
    }

    pub(crate) fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        self.application.borrow_mut().set_scene(scene);
    }

    pub(crate) fn set_current_fps(&self, fps: usize) {
        self.application.borrow_mut().set_current_fps(fps);
    }

    pub fn default_label_option(&self) -> Option<LabelOption> {
        self.application.borrow().default_label_option()
    }

    pub fn set_default_label_option(&self, option: &LabelOption) {
        self.application.borrow_mut().set_default_label_option(option);
    }

    pub fn current_fps(&self) -> usize {
        self.application.borrow().current_fps()
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

    pub(crate) fn set_render_point(&self, id: &NodeId, render_point: &Point) {
        self.node.borrow_mut().set_render_point(id, render_point);
    }

    pub(crate) fn get_render_point(&self, id: &NodeId) -> Option<Point> {
        self.node.borrow().get_render_point(id)
    }

    pub(crate) fn clear_render_points(&self) {
        self.node.borrow_mut().clear_render_points();
    }

    pub fn destroy_node(&self, id: &NodeId) {
        self.node.borrow_mut().destroy(id);
    }

    pub fn add_alias(&self, name: &str, path: &str) {
        self.resource.borrow_mut().add_alias(name, path);
    }

    pub fn load_plain_data(&self, path: &str) -> Rc<Vec<u8>> {
        self.resource.borrow_mut().load_plain_data(path)
    }

    pub fn load_texture(&self, path: &str) -> Rc<RTexture> {
        self.resource.borrow_mut().load_texture(path)
    }

    pub fn load_font(&self, option: &LabelOption) -> Rc<RFont> {
        self.resource.borrow_mut().load_font(option)
    }

}
