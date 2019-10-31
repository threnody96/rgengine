use std::rc::Rc;
use ::resource::{ RTexture, RFont };
use ::node::{ NodeId, NodeLike, AddChildOption };
use ::util::{ director, Point, AnchorPoint, Size };
use sdl2::pixels::{ Color };

pub trait NodeDelegate {

    fn get_size(&self) -> Size;

    fn update(&self, parent: Rc<dyn NodeLike>);

    fn render(&self, parent: Rc<dyn NodeLike>) { }

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

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
        director(|d| d.get_nodelike(&self.id()))
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.node().add_child(node, option);
    }

    fn get_children(&self) -> Vec<NodeId> {
        self.node().get_children()
    }

    fn set_position(&self, position: &Point) {
        self.node().set_position(position);
    }

    fn get_position(&self) -> Point {
        self.node().get_position()
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

    fn prepare_render_tree(&self, parent: &Option<Rc<dyn NodeLike>>) {
        director(|d| d.prepare_render_tree(parent, self.node()));
    }

    fn render_texture(&self, texture: Rc<RTexture>) {
        director(|d| d.render_texture(self.node(), texture));
    }

    fn render_label(&self, text: &str, font: Rc<RFont>, color: &Color) {
        director(|d| d.render_label(self.node(), text, font, color));
    }

}

