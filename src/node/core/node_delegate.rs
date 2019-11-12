use std::rc::Rc;
use std::any::Any;
use ::resource::{ Texture, Font };
use ::node::{ Node, NodeId, NodeLike, AddChildOption, ConflictType };
use ::action::{ ActionLike };
use ::util::{ director };
use ::util::parameter::{ Point, AnchorPoint, Size, Color, Scale, Opacity, Rotation };

pub trait NodeDelegate: Any {

    fn get_size(&self) -> Size;

    fn update(&self);

    fn render(&self) { }

    fn before_add_child(&self, child: Rc<dyn NodeLike>) { }

    fn before_be_added_child(&self, parent: Rc<dyn NodeLike>){ }

    fn use_cache(&self) -> bool {
        false
    }

    fn clear_cache(&self) {
        self.node().inner_clear_cache();
    }

    fn id(&self) -> NodeId {
        NodeId::new(format!("{:p}", self))
    }

    fn node(&self) -> Rc<dyn NodeLike> {
        director::get_nodelike(&self.id())
    }

    fn set_conflict_type(&self, conflict_type: ConflictType) {
        self.node().inner_set_conflict_type(conflict_type);
    }

    fn add_child<A>(&self, node: Rc<dyn NodeLike>, option: A) where A: Into<AddChildOption> {
        self.node().inner_add_child(node, option.into());
    }

    // fn get_parent(&self) -> Option<Rc<dyn NodeLike>> {
    //     self.node().get_parent()
    // }

    // fn get_children(&self) -> Vec<Rc<dyn NodeLike>> {
    //     self.node().get_children()
    // }

    fn set_position<A>(&self, position: A) where A: Into<Point> {
        self.node().inner_set_position(position.into());
    }

    fn get_position(&self) -> Point {
        self.node().inner_get_position()
    }

    fn set_visible(&self, visible: bool) {
        self.node().inner_set_visible(visible);
    }

    fn get_visible(&self) -> bool {
        self.node().inner_get_visible()
    }

    fn set_rotation<A>(&self, rotation: A) where A: Into<Rotation> {
        self.node().inner_set_rotation(rotation.into());
    }

    fn get_rotation(&self) -> Rotation {
        self.node().inner_get_rotation()
    }

    fn set_scale<A>(&self, scale: A) where A: Into<Scale> {
        self.node().inner_set_scale(scale.into());
    }

    fn get_scale(&self) -> Scale {
        self.node().inner_get_scale()
    }

    fn set_opacity<A>(&self, opacity: A) where A: Into<Opacity> {
        self.node().inner_set_opacity(opacity.into());
    }

    fn get_opacity(&self) -> Opacity {
        self.node().inner_get_opacity()
    }

    fn set_anchor_point<A>(&self, anchor_point: A) where A: Into<AnchorPoint> {
        self.node().inner_set_anchor_point(anchor_point.into());
    }

    fn get_anchor_point(&self) -> AnchorPoint {
        self.node().inner_get_anchor_point()
    }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        None
    }

    fn render_texture(&self, texture: Rc<Texture>) {
        director::render_texture(self.node(), texture);
    }

    fn render_label(&self, text: &str, font: Rc<Font>, color: &Color) {
        director::render_label(self.node(), text, font, color);
    }

    fn render_round(&self, color: &Color) {
        director::render_round(self.node(), color);
    }

    fn is_mouse_hover(&self) -> bool {
        self.node().inner_is_mouse_hover()
    }

    fn is_conflict(&self, other: Rc<dyn NodeLike>) -> bool {
        self.node().inner_is_conflict(other)
    }

    fn run_action(&self, action: Rc<dyn ActionLike>) {
        self.node().inner_run_action(action);
    }

    fn destroy(&self) {
        self.node().inner_destroy();
    }

}

