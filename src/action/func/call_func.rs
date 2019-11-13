use std::rc::Rc;
use ::node::{ Node, NodeLike, NodeDelegate };
use ::action::{ ParentActionDelegate, ActionLike, ParentAction, ActionStatus };
use ::util::{ director };
use ::util::easing::{ EasingFunction };

pub struct CallFunc<T> where T: NodeDelegate {
    callback: Rc<dyn Fn(Rc<Node<T>>) -> ()>
}

impl <T> CallFunc<T> where T: NodeDelegate {

    pub fn create(callback: Rc<dyn Fn(Rc<Node<T>>) -> ()>) -> Rc<ParentAction<CallFunc<T>>> {
        ParentAction::create(Self {
            callback: callback.clone()
        })
    }

}

impl <T> ParentActionDelegate for CallFunc<T> where T: NodeDelegate {

    fn run(&self, node: Rc<dyn NodeLike>, _easing: Option<Rc<dyn EasingFunction>>) -> ActionStatus {
        if let Some(n) = director::get_node::<T>(&node.inner_id()) {
            (&self.callback)(n);
        }
        ActionStatus::Finish
    }

    fn children(&self) -> Vec<Rc<dyn ActionLike>> {
        Vec::new()
    }

}
