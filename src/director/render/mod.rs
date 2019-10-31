use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ::node::{ NodeId, NodeLike, LabelOption };
use ::resource::{ RTexture, RFont, ResourceKey };
use ::application::{ Application, ResolutionPolicy };
use ::util::{ context, Size, Rect };
use ::director::resource::{ ResourceDirector };
use sdl2::render::{ Texture, BlendMode };
use sdl2::pixels::{ PixelFormatEnum, Color };

#[derive(Clone)]
pub enum RenderOperation {
    Image(Rc<RTexture>),
    Label(String, Rc<RFont>, Color)
}

pub struct RenderTree {
    node: Rc<dyn NodeLike>,
    operations: RefCell<Vec<RenderOperation>>,
    children: RefCell<Vec<Rc<RenderTree>>>
}

impl RenderTree {

    pub fn new(node: Rc<dyn NodeLike>) -> Rc<Self> {
        Rc::new(Self {
            node: node,
            operations: RefCell::new(Vec::new()),
            children: RefCell::new(Vec::new())
        })
    }

    pub fn push_operation(&self, operation: RenderOperation) {
        self.operations.borrow_mut().push(operation);
    }

    pub fn push_child(&self, child: Rc<RenderTree>) {
        self.children.borrow_mut().push(child);
    }

}

pub struct RenderDirector<'a> {
    application: Option<Rc<dyn Application>>,
    resource: ResourceDirector<'a>,
    resolution_size: Size,
    resolution_policy: ResolutionPolicy,
    render_canvas_dest: Option<Rect>,
    render_tree_nodes: HashMap<NodeId, Rc<RenderTree>>,
    render_tree: Option<Rc<RenderTree>>,
}

impl <'a> RenderDirector<'a> {

    pub fn new() -> Self {
        Self {
            application: None,
            resource: ResourceDirector::new(),
            resolution_size: Size::new(0, 0),
            resolution_policy: ResolutionPolicy::ExactFit,
            render_canvas_dest: None,
            render_tree_nodes: HashMap::new(),
            render_tree: None,
        }
    }

    fn generate_render_canvas_dest(app: Rc<dyn Application>) -> Option<Rect> {
        let (window_size, rsize, policy) = (app.window_size(), app.resolution_size(), app.resolution_policy());
        if &policy == &ResolutionPolicy::ExactFit { return None; }
        let per_size = (window_size.width() as f32 / rsize.width() as f32, window_size.height() as f32 / rsize.height() as f32);
        let use_per_size = Self::choice_per_size_from_policy(per_size.0, per_size.1, policy);
        let dest_size = ((rsize.width() as f32 * use_per_size).round() as i32, (rsize.height() as f32 * use_per_size).round() as i32);
        let dest_center = (dest_size.0 / 2, dest_size.1 / 2);
        let real_center = (window_size.width() as i32 / 2, window_size.height() as i32 / 2);
        let render_to = (real_center.0 - dest_center.0, real_center.1 - dest_center.1);
        Some(Rect::new(render_to.0, render_to.1, dest_size.0 as u32, dest_size.1 as u32))
    }

    fn choice_per_size_from_policy(per_width: f32, per_height: f32, policy: ResolutionPolicy) -> f32 {
        match policy {
            ResolutionPolicy::ShowAll => { if per_width > per_height { per_height } else { per_width } },
            ResolutionPolicy::NoBorder => { if per_width > per_height { per_width } else { per_height } },
            ResolutionPolicy::FixedWidth => { per_width },
            ResolutionPolicy::FixedHeight => { per_height },
            ResolutionPolicy::ExactFit => { panic!("不到達コード"); },
        }
    }

    pub fn set_application(&mut self, application: Rc<dyn Application>) {
        self.application = Some(application.clone());
        self.resolution_size = application.resolution_size();
        self.resolution_policy = application.resolution_policy();
        self.render_canvas_dest = Self::generate_render_canvas_dest(application);
    }

    pub fn add_alias(&mut self, name: &str, path: &str) {
        self.resource.add_alias(name, path);
    }

    pub fn load_plain_data(&mut self, path: &str) -> Rc<Vec<u8>> {
        self.resource.load_plain_data(path)
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<RTexture> {
        self.resource.load_texture(path)
    }

    pub fn load_font(&mut self, option: &LabelOption) -> Rc<RFont> {
        self.resource.load_font(option)
    }

    pub fn prepare_render_tree(&mut self, parent: &Option<Rc<dyn NodeLike>>, node: Rc<dyn NodeLike>) {
        let id = node.id();
        if self.render_tree_nodes.get(&id).is_none() {
            let tree_node = RenderTree::new(node.clone());
            self.render_tree_nodes.insert(id.clone(), tree_node.clone());
            match parent {
                None => {
                    self.render_tree = Some(tree_node);
                },
                Some(p) => {
                    self.render_tree_nodes.get(&p.id()).unwrap().push_child(tree_node.clone());
                }
            }
        }
    }

    pub fn render_texture(&mut self, node: Rc<dyn NodeLike>, texture: Rc<RTexture>) {
        let tree = self.render_tree_nodes.get(&node.id()).unwrap();
        tree.push_operation(RenderOperation::Image(texture));
    }

    pub fn render_label(&mut self, node: Rc<dyn NodeLike>, text: &str, font: Rc<RFont>, color: &Color) {
        let tree = self.render_tree_nodes.get(&node.id()).unwrap();
        tree.push_operation(RenderOperation::Label(text.to_owned(), font, color.clone()));
    }

    pub fn measure_label_size(&self, text: &str, font: Rc<RFont>) -> Size {
        let f = self.resource.load_font_from_resource_key(font);
        let surface = f.render(text).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        Size::new(surface.width(), surface.height())
    }

    pub fn update_resolution_size(&mut self) {
        let application = self.application.clone().unwrap();
        let (resolution_size, resolution_policy) = (application.resolution_size(), application.resolution_policy());
        if self.resolution_size != resolution_size || self.resolution_policy != resolution_policy {
            self.resolution_size = resolution_size;
            self.resolution_policy = resolution_policy;
            self.render_canvas_dest = Self::generate_render_canvas_dest(application);
        }
    }

    fn create_sub_canvas(&self, node: Rc<dyn NodeLike>) -> Texture<'a> {
        let canvas_size = node.get_size();
        let mut texture = context(|c| {
            c.texture_creator.create_texture_target(
                Some(PixelFormatEnum::RGBA8888),
                canvas_size.width(),
                canvas_size.height()
            ).unwrap()
        });
        texture.set_blend_mode(BlendMode::Blend);
        texture
    }

    fn render_inner_canvas(&mut self, render_tree: Rc<RenderTree>) -> Option<Rc<Texture<'a>>> {
        let node = render_tree.node.clone();
        if !node.get_visible() { return None; }
        if let Some(cache) = self.load_by_render_cache(node.clone()) {
            return Some(cache);
        }
        let children: Vec<(Rc<dyn NodeLike>, Option<Rc<Texture<'a>>>)> = render_tree.children.borrow().iter().map(|child| {
            (child.node.clone(), self.render_inner_canvas(child.clone()))
        }).collect();
        let mut sub_canvas = self.create_sub_canvas(node.clone());
        let canvas = context(|c| &mut c.canvas);
        canvas.with_texture_canvas(&mut sub_canvas, |c| {
            c.set_blend_mode(BlendMode::Blend);
            c.clear();
            for operation in render_tree.operations.borrow().iter() {
                let t = self.exec_operation(operation);
                let query = t.query();
                c.copy(&t, None, Some(Rect::new(0, 0, query.width, query.height))).unwrap();
            }
            for (child_node, child_texture) in children {
                if let Some(ct) = child_texture {
                    let point = child_node.get_render_point();
                    let angle = child_node.get_rotation();
                    let query = ct.query();
                    c.copy_ex(&ct, None, Some(Rect::new(point.x(), point.y(), query.width, query.height)), angle, None, false, false).unwrap();
                }
            }
        }).unwrap();
        sub_canvas.set_alpha_mod(node.get_opacity());
        let r = Rc::new(sub_canvas);
        if node.use_cache() {
            let key = self.resource.set_render_cache(r.clone());
            node.set_cache(Some(key));
        }
        Some(r)
    }

    fn load_by_render_cache(&self, node: Rc<dyn NodeLike>) -> Option<Rc<Texture<'a>>> {
        if !node.use_cache() { return None; }
        if let Some(key) = node.get_cache() {
            return Some(self.resource.get_render_cache(&key));
        }
        None
    }

    fn exec_operation(&self, operation: &RenderOperation) -> Rc<Texture<'a>> {
        match operation {
            RenderOperation::Image(texture) => {
                self.resource.load_texture_from_resource_key(texture.clone())
            },
            RenderOperation::Label(text, font, color) => {
                let f = self.resource.load_font_from_resource_key(font.clone());
                let surface = f.render(text.as_str()).blended(*color).unwrap();
                let texture = context(|c| c.texture_creator.create_texture_from_surface(surface)).unwrap();
                Rc::new(texture)
            }
        }
    }

    pub fn render_canvas(&mut self) {
        if let Some(render_tree) = self.render_tree.clone() {
            let texture = self.render_inner_canvas(render_tree);
            context(|c| {
                c.canvas.clear();
                if let Some(t) = texture {
                    c.canvas.copy(&t, None, self.render_canvas_dest.clone()).unwrap();
                }
                c.canvas.present();
            });
        } else {
            context(|c| {
                c.canvas.clear();
                c.canvas.present();
            });
        }
        self.render_tree = None;
        self.render_tree_nodes = HashMap::new();
    }

    pub fn destroy_render_cache(&mut self, key: &ResourceKey) {
        self.resource.destroy_render_cache(key);
    }

}
