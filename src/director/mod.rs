pub mod node;
pub mod application;
pub mod resource;

use std::rc::Rc;
use std::any::Any;
use std::cell::RefCell;
use self::node::{ NodeDirector };
use self::application::{ ApplicationDerector };
use self::resource::{ ResourceDirector };
use ::node::{ Node, NodeDelegate, SceneLike, NodeId, NodeLike, LabelTextOption };
use ::application::{ AppDelegate, ResolutionPolicy, ResolutionSize };
use ::util::{ must, Size };
use ggez::{ Context };
use ggez::graphics::{ Scale, Image, Font, Color };
use serde_json::{ Value };

pub struct Director {
    node: NodeDirector,
    application: ApplicationDerector,
    resource: ResourceDirector,
    context: RefCell<Option<Context>>
}

impl Director {

    pub fn new() -> Self {
        Self {
            node: NodeDirector::new(),
            application: ApplicationDerector::new(),
            resource: ResourceDirector::new(),
            context: RefCell::new(None)
        }
    }

    pub fn run_with_scene(&self, app_delegate: Rc<dyn AppDelegate>, scene: Rc<dyn SceneLike>) {
        let (ctx, mut event_loop) = self.application.init(app_delegate, scene);
        self.set_context(ctx);
        self.application.run(&mut event_loop);
    }

    pub fn set_context(&self, ctx: Context) {
        self.context.replace(Some(ctx));
    }

    pub fn with_context<T, R>(&self, callback: T) -> R where T: FnOnce(&mut Context) -> R {
        let mut ctx = self.context.borrow_mut();
        if ctx.is_none() { must::<String, String>(Err("ゲームが実行されていません".to_owned())); }
        callback(ctx.as_mut().unwrap())
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.application.get_scene()
    }

    pub fn set_scene(&self, scene: Rc<dyn SceneLike>) {
        if self.application.get_scene().id() != scene.id() {
            self.destroy_node(self.application.get_scene().id());
        }
        self.application.set_scene(scene);
    }

    pub fn add_font_size(&self, name: String, scale: f32) {
        self.application.add_font_size(name, scale);
    }

    pub fn get_font_size(&self, name: String) -> Option<Scale> {
        self.application.get_font_size(name)
    }

    pub fn add_font(&self, name: String, path: String) {
        self.application.add_font(name, path);
    }

    pub fn get_font(&self, name: String) -> Option<String> {
        self.application.get_font(name)
    }

    pub fn add_color(&self, name: String, color: Color) {
        self.application.add_color(name, color);
    }

    pub fn get_color(&self, name: String) -> Option<Color> {
        self.application.get_color(name)
    }

    pub fn get_default_label_option(&self) -> LabelTextOption {
        self.application.get_default_label_option()
    }

    pub fn get_visible_size(&self) -> Size {
        self.application.get_visible_size()
    }

    pub fn set_visible_size(&self, size: Size) {
        self.application.set_visible_size(size);
    }

    pub fn get_resolution_size(&self) -> ResolutionSize {
        self.application.get_resolution_size()
    }

    pub fn set_resolution_size(&self, size: Size, policy: ResolutionPolicy) {
        self.application.set_resolution_size(size, policy);
    }

    pub fn get_display_stats(&self) -> bool {
        self.application.get_display_stats()
    }

    pub fn set_display_stats(&self, display_stats: bool) {
        self.application.set_display_stats(display_stats);
    }

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.node.register_node(node);
    }

    pub fn get_node<T>(&self, id: NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        self.node.get_node(id)
    }

    pub fn get_nodelike(&self, id: NodeId) -> Option<Rc<dyn NodeLike>> {
        self.node.get_nodelike(id)
    }

    pub fn update_node(&self, id: NodeId) {
        self.node.update(id);
    }

    pub fn render_node(&self, id: NodeId) {
        self.node.render(id);
    }

    pub fn destroy_node(&self, id: NodeId) {
        self.node.destroy(id);
    }

    pub fn load_plain_data(&self, path: String) -> Rc<Vec<u8>> {
        self.resource.load_plain_data(path)
    }

    pub fn load_string(&self, path: String) -> Rc<String> {
        self.resource.load_string(path)
    }

    pub fn load_json(&self, path: String) -> Rc<Value> {
        self.resource.load_json(path)
    }

    pub fn load_image(&self, path: String) -> Rc<Image> {
        self.with_context(|ctx| {
            self.resource.load_image(ctx, path)
        })
    }

    pub fn load_font(&self, path: String) -> Rc<Font> {
        self.with_context(|ctx| {
            self.resource.load_font(ctx, path)
        })
    }

}
