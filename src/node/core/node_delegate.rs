use std::rc::Rc;
use std::any::Any;
use ::resource::{ Texture, Font };
use ::node::{ Node, NodeId, NodeLike, AddChildOption, ConflictType, RunActionOption };
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

    fn get_parent<A>(&self) -> Option<Rc<Node<A>>> where A: NodeDelegate {
        if let Some(parent_id) = self.node().inner_get_parent_id() {
            return director::get_node::<A>(&parent_id);
        }
        None
    }

    fn get_child<A, B>(&self, name: A) -> Option<Rc<Node<B>>> where A: Into<String>, B: NodeDelegate {
        let n = name.into();
        if let Some(child_id) = self.node().inner_get_child_id(&n) {
            return director::get_node::<B>(&child_id);
        }
        None
    }

    fn get_children<A>(&self) -> Vec<Rc<Node<A>>> where A: NodeDelegate {
        let mut output: Vec<Rc<Node<A>>> = Vec::new();
        for child_id in self.node().inner_get_children_ids() {
            if let Some(child) = director::get_node::<A>(&child_id) {
                output.push(child);
            }
        }
        output
    }

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

    fn is_additive_blend(&self) -> bool {
        self.node().inner_is_additive_blend()
    }

    fn set_additive_blend(&self, additive_blend: bool) {
        self.node().inner_set_additive_blend(additive_blend);
    }

    fn get_fixed_anchor_point(&self) -> Option<AnchorPoint> {
        None
    }

    fn render_texture(&self, texture: Rc<Texture>) {
        director::render_texture(self.node(), texture);
    }

    fn render_label<A, B>(&self, text: A, font: Rc<Font>, color: B) where A: Into<String>, B: Into<Color> {
        let t = text.into();
        let c = color.into();
        director::render_label(self.node(), &t, font, &c);
    }

    fn render_round<A>(&self, color: A) where A: Into<Color> {
        let c = color.into();
        director::render_round(self.node(), &c);
    }

    fn render_square<A>(&self, color: A) where A: Into<Color> {
        let c = color.into();
        director::render_square(self.node(), &c);
    }

    fn is_mouse_hover(&self) -> bool {
        self.node().inner_is_mouse_hover()
    }

    fn is_conflict(&self, other: Rc<dyn NodeLike>) -> bool {
        self.node().inner_is_conflict(other)
    }

    fn run_action<A>(&self, action: Rc<dyn ActionLike>, option: A) where A: Into<RunActionOption> {
        self.node().inner_run_action(action, option.into());
    }

    fn get_action<A>(&self, name: A) -> Option<Rc<dyn ActionLike>> where A: Into<String> {
        self.node().inner_get_action(name.into())
    }

    fn destroy(&self) {
        self.node().inner_destroy();
    }

}

