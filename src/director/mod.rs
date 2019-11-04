mod application;
mod node;
mod resource;
mod render;
mod input;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use ::application::{ Application };
use ::util::parameter::{ Size };
use ::node::{ Node, NodeLike, NodeDelegate, NodeId };
use ::node::scene::{ SceneLike };
use ::node::label::{ LabelOption };
use ::resource::{ Texture, Font, ResourceKey };
use self::application::ApplicationDirector;
use self::node::NodeDirector;
use self::render::RenderDirector;
use sdl2::pixels::{ Color };
use rand::distributions::{ Standard, Distribution };

pub struct Director<'a> {
    application: RefCell<ApplicationDirector>,
    node: RefCell<NodeDirector>,
    render: RefCell<RenderDirector<'a>>
}

impl <'a> Director<'a> {

    pub fn new() -> Self {
        Self {
            application: RefCell::new(ApplicationDirector::new()),
            node: RefCell::new(NodeDirector::new()),
            render: RefCell::new(RenderDirector::new())
        }
    }

    pub fn window_size(&self) -> Size {
        self.application.borrow().window_size()
    }

    pub fn get_resolution_size(&self) -> Size {
        self.application.borrow().get_resolution_size()
    }

    pub fn rand<T>(&self) -> T where Standard: Distribution<T> {
        self.application.borrow_mut().rand()
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
        self.application.borrow_mut().set_application(application.clone());
        self.render.borrow_mut().set_application(application.clone());
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

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.node.borrow_mut().register_node(node);
    }

    pub fn get_node<T>(&self, id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        self.node.borrow().get_node(id)
    }

    pub fn get_nodelike(&self, id: &NodeId) -> Rc<dyn NodeLike> {
        self.node.borrow().get_nodelike(id)
    }

    pub fn destroy_node(&self, id: &NodeId) {
        self.node.borrow_mut().destroy(id);
    }

    pub fn measure_label_size(&self, text: &str, font: Rc<Font>) -> Size {
        self.render.borrow().measure_label_size(text, font)
    }

    pub fn add_alias(&self, name: &str, path: &str) {
        self.render.borrow_mut().add_alias(name, path);
    }

    pub fn load_plain_data(&self, path: &str) -> Rc<Vec<u8>> {
        self.render.borrow_mut().load_plain_data(path)
    }

    pub fn load_texture(&self, path: &str) -> Rc<Texture> {
        self.render.borrow_mut().load_texture(path)
    }

    pub fn load_font(&self, option: &LabelOption) -> Rc<Font> {
        self.render.borrow_mut().load_font(option)
    }

    pub fn prepare_render_tree(&self, parent: &Option<Rc<dyn NodeLike>>, node: Rc<dyn NodeLike>) {
        self.render.borrow_mut().prepare_render_tree(parent, node);
    }

    pub fn render_texture(&self, node: Rc<dyn NodeLike>, texture: Rc<Texture>) {
        self.render.borrow_mut().render_texture(node, texture);
    }

    pub fn render_label(&self, node: Rc<dyn NodeLike>, text: &str, font: Rc<Font>, color: &Color) {
        self.render.borrow_mut().render_label(node, text, font, color);
    }

    pub fn update_resolution_size(&self) {
        self.render.borrow_mut().update_resolution_size();
    }

    pub fn render_canvas(&self) {
        self.render.borrow_mut().render_canvas();
    }

    pub fn destroy_render_cache(&self, key: &ResourceKey) {
        self.render.borrow_mut().destroy_render_cache(key);
    }

}
