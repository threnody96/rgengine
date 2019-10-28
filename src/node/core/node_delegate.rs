use std::rc::Rc;
use ::resource::{ RTexture, RFont };
use ::node::{ NodeId, NodeLike, AddChildOption };
use ::util::{ director, render, Point };
use sdl2::pixels::{ Color };

pub trait NodeDelegate {

    fn update(&self);

    fn render(&self, parent: Option<Rc<dyn NodeLike>>);

    fn before_add_child(&self) { }

    fn before_be_added_child(&self) { }

    fn id(&self) -> NodeId {
        NodeId::new(format!("{:p}", self))
    }

    fn node(&self) -> Rc<dyn NodeLike> {
        director(|d| {
            let id = self.id();
            d.get_nodelike(&id).unwrap()
        })
    }

    fn add_child(&self, node: Rc<dyn NodeLike>, option: AddChildOption) {
        self.node().add_child(node, option);
    }

    fn get_children(&self) -> Vec<NodeId> {
        self.node().get_children()
    }

    fn generate_position_with_parent(&self, parent: &Option<Rc<dyn NodeLike>>) -> Point {
        let mut position = self.node().get_position();
        if let Some(p) = parent.clone() {
            let parent_position = p.get_position();
            position = Point::new(position.x() + parent_position.x(), position.y() + parent_position.y());
        }
        position
    }

    fn render_texture(&self, parent: &Option<Rc<dyn NodeLike>>, texture: &RTexture) {
        let position = self.generate_position_with_parent(parent);
        render(|r| r.render_texture(position, texture));
    }

    fn render_label(&self, parent: &Option<Rc<dyn NodeLike>>, text: &str, font: &RFont, color: &Color) {
        let position = self.generate_position_with_parent(parent);
        render(|r| r.render_label(position, text, font, color));
    }

}

