use std::rc::Rc;
use std::cmp::Ordering;
use std::any::Any;
use ::util::{ Point };

pub struct NodeChild {
    pub node: Rc<Box<dyn Node>>,
    pub z_index: i32,
    pub inner_z_index: u32,
    pub tags: Vec<String>
}

#[derive(Clone)]
pub struct AddChildOption {
    pub z_index: i32,
    pub tags: Vec<String>
}

impl Default for AddChildOption {

    fn default() -> Self {
        AddChildOption {
            z_index: 0,
            tags: Vec::new()
        }
    }

}

pub trait NodeCore: Any {

    fn get_children_mut_core(&mut self) -> &mut Vec<NodeChild>;

    fn get_children_core(&self) -> &Vec<NodeChild>;

    fn set_position_core(&mut self, position: Point);

    fn set_anchor_point_core(&mut self, anchor_point: Point);

    fn create_node_child(&self, child: Rc<Box<dyn Node>>, option: AddChildOption) -> NodeChild;

    fn sort_children(&mut self);

}

pub trait Node: NodeCore {

    fn render_self(&self);

    fn add_child_filter(&self, child: Rc<Box<dyn Node>>, option: AddChildOption) -> bool {
        true
    }

    fn add_child(&mut self, child: Rc<Box<dyn Node>>, option: AddChildOption) -> Result<(), String> {
        if (!self.add_child_filter(child.clone(), option.clone())) {
            return Err("フィルタリングされました".to_owned())
        }
        let node_child = self.create_node_child(child, option);
        self.get_children_mut_core().push(node_child);
        self.sort_children();
        Ok(())
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
            child.node.render();
        }
    }

}

macro_rules! define_node_impl {
    ($t: ident) => (
        use std::rc::Rc;
        use std::cmp::Ordering;
        use ::util::{ Point };
        use ::node::{ Node, NodeCore, NodeChild, AddChildOption };

        impl NodeCore for $t {

            fn get_children_mut_core(&mut self) -> &mut Vec<NodeChild> {
                &mut self.children
            }

            fn get_children_core(&self) -> &Vec<NodeChild> {
                &self.children
            }

            fn set_position_core(&mut self, position: Point) {
                self.position = position;
            }

            fn set_anchor_point_core(&mut self, anchor_point: Point) {
                self.anchor_point = anchor_point;
            }

            fn create_node_child(&self, child: Rc<Box<dyn Node>>, option: AddChildOption) -> NodeChild {
                let mut inner_z_index: u32 = 0;
                for node_child in self.get_children_core() {
                    if node_child.z_index == option.z_index && node_child.inner_z_index > inner_z_index {
                        inner_z_index = node_child.inner_z_index;
                    }
                }
                NodeChild {
                    node: child,
                    z_index: option.z_index,
                    inner_z_index: inner_z_index + 1,
                    tags: option.tags
                }
            }

            fn sort_children(&mut self) {
                self.get_children_mut_core().sort_by(|a, b| {
                    let p = a.z_index.partial_cmp(&b.z_index).unwrap();
                    if p != Ordering::Equal { return p; }
                    a.inner_z_index.partial_cmp(&b.inner_z_index).unwrap()
                });
            }

        }
    );
}

#[macro_export]
macro_rules! define_node {
    ($t:ident) => (
        define_node!($t, );
    );
    ($t:ident, $($e: ident: $ty: ty),*) => (
        pub struct $t {
            position: Point,
            anchor_point: Point,
            children: Vec<NodeChild>,
            $(pub $e: $ty),*
        }

        impl $t {

            fn create($($e: $ty),*) -> Self {
                Self {
                    position: Point { x: 0.0f32, y: 0.0f32 },
                    anchor_point: Point { x: 0.0f32, y: 0.0f32 },
                    children: Vec::new(),
                    $($e: $e),*
                }
            }

        }

        define_node_impl!($t);
    );
}
