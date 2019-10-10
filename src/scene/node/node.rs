use ::util::{ Point };

pub trait NodeCore {

    fn get_children_mut_core(&mut self) -> &mut Vec<Box<dyn Node>>;

    fn get_children_core(&self) -> &Vec<Box<dyn Node>>;

    fn set_position_core(&mut self, position: Point);

    fn set_anchor_point_core(&mut self, anchor_point: Point);

}

pub trait Node: NodeCore {

    fn render_self(&self);

    fn add_child(&mut self, child: Box<dyn Node>) {
        self.get_children_mut_core().push(child);
    }

    fn set_position(&mut self, position: Point) {
        self.set_position_core(position);
    }

    fn set_anchor_point(&mut self, anchor_point: Point) {
        self.set_anchor_point_core(anchor_point);
    }

    fn render(&self) {
        self.render_self();
        for child in self.get_children_core() {
            child.render();
        }
    }

}

macro_rules! define_node_impl {
    ($t: ident) => (
        impl NodeCore for $t {

            fn get_children_mut_core(&mut self) -> &mut Vec<Box<dyn Node>> {
                &mut self.children
            }

            fn get_children_core(&self) -> &Vec<Box<dyn Node>> {
                &self.children
            }

            fn set_position_core(&mut self, position: Point) {
                self.position = position;
            }

            fn set_anchor_point_core(&mut self, anchor_point: Point) {
                self.anchor_point = anchor_point;
            }

        }
    );
}

#[macro_export]
macro_rules! define_node {
    ($t:ident) => (
        struct $t {
            position: Point,
            anchor_point: Point,
            children: Vec<Box<dyn Node>>,
        }
        define_node_impl!($t);
    );
    ($t:ident, $($e: ident: $ty: ty),*) => (
        struct $t {
            position: Point,
            anchor_point: Point,
            children: Vec<Box<dyn Node>>,
            $($e: $ty),*
        }
        define_node_impl!($t);
    );
}

define_node!(Hoge);

impl Node for Hoge {

    fn render_self(&self) {

    }

}

