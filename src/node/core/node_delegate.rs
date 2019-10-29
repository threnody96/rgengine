use std::rc::Rc;
use ::resource::{ RTexture, RFont };
use ::node::{ NodeId, NodeLike, AddChildOption };
use ::util::{ director, render, Point, AnchorPoint, Size };
use sdl2::pixels::{ Color };

pub trait NodeDelegate {

    fn get_size(&self) -> Size;

    fn update(&self);

    fn render(&self, parent: Option<Rc<dyn NodeLike>>);

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

    fn id(&self) -> NodeId {
        NodeId::new(format!("{:p}", self))
    }

    fn node(&self) -> Rc<dyn NodeLike> {
        director(|d| {
            d.get_nodelike(&self.id()).unwrap()
        })
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.node().add_child(node, option);
    }

    fn get_children(&self) -> Vec<NodeId> {
        self.node().get_children()
    }

    fn generate_position_with_parent(&self, parent: &Option<Rc<dyn NodeLike>>) -> Point {
        let node = self.node();
        let mut position = node.get_render_point();
        if let Some(p) = parent.clone() {
            if let Some(parent_position) = director(|d| d.get_render_point(&p.id())) {
                position = Point::new(position.x() + parent_position.x(), position.y() + parent_position.y());
            }
        }
        director(|d| d.set_render_point(&self.id(), &position));
        position
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

    fn render_texture(&self, parent: &Option<Rc<dyn NodeLike>>, texture: Rc<RTexture>) {
        let position = self.generate_position_with_parent(parent);
        render(|r| r.render_texture(position, texture));
    }

    fn render_label(&self, parent: &Option<Rc<dyn NodeLike>>, text: &str, font: Rc<RFont>, color: &Color) {
        let position = self.generate_position_with_parent(parent);
        render(|r| r.render_label(position, text, font, color));
    }

}

