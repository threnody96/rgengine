use std::rc::Rc;
use ::resource::{ Texture, Font };
use ::node::{ NodeId, NodeLike, AddChildOption };
use ::action::{ ActionLike };
use ::util::{ director };
use ::util::parameter::{ Point, AnchorPoint, Size, Color };

pub trait NodeDelegate {

    fn get_size(&self) -> Size;

    fn update(&self);

    fn render(&self) { }

    fn before_add_child(&self, child: Rc<dyn NodeLike>) { }

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>) { }

    fn use_cache(&self) -> bool {
        false
    }

    fn clear_cache(&self) {
        self.node().clear_cache();
    }

    fn id(&self) -> NodeId {
        NodeId::new(format!("{:p}", self))
    }

    fn node(&self) -> Rc<dyn NodeLike> {
        director::get_nodelike(&self.id())
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.node().add_child(node, option);
    }

    fn get_parent(&self) -> Option<Rc<dyn NodeLike>> {
        self.node().get_parent()
    }

    fn get_children(&self) -> Vec<Rc<dyn NodeLike>> {
        self.node().get_children()
    }

    fn set_position(&self, position: &Point) {
        self.node().set_position(position);
    }

    fn get_position(&self) -> Point {
        self.node().get_position()
    }

    fn set_visible(&self, visible: bool) {
        self.node().set_visible(visible);
    }

    fn get_visible(&self) -> bool {
        self.node().get_visible()
    }

    fn set_rotation(&self, rotation: f64) {
        self.node().set_rotation(rotation);
    }

    fn get_rotation(&self) -> f64 {
        self.node().get_rotation()
    }

    fn set_scale(&self, scale: f64) {
        self.node().set_scale(scale);
    }

    fn get_scale(&self) -> f64 {
        self.node().get_scale()
    }

    fn set_anchor_point(&self, anchor_point: &AnchorPoint) {
        self.node().set_anchor_point(anchor_point);
    }

    fn get_anchor_point(&self) -> AnchorPoint {
        self.node().get_anchor_point()
    }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        None
    }

    fn prepare_render_tree(&self) {
        director::prepare_render_tree(self.get_parent(), self.node());
    }

    fn render_texture(&self, texture: Rc<Texture>) {
        director::render_texture(self.node(), texture);
    }

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color) {
        director::render_label(self.node(), text, font, color);
    }

    fn is_mouse_hover(&self) -> bool {
        self.node().is_mouse_hover()
    }

    fn is_conflict(&self, other: Rc<dyn NodeLike>) -> bool {
        self.node().is_conflict(other)
    }

    fn run_action(&self, action: Rc<dyn ActionLike>) {
        self.node().run_action(action);
    }

}

