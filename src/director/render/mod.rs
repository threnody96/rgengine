use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ::node::{ NodeId, NodeLike };
use ::node::scene::{ SceneLike };
use ::node::scene::transition::{ SceneTransition, TransitionStatus };
use ::node::label::{ OneLineLabelOption };
use ::resource::{ ResourceKey };
use ::application::{ Application, ResolutionPolicy };
use ::util::{ with_context };
use ::util::parameter::{ Size, Rect, Point, Circle };
use ::director::resource::{ ResourceDirector };
use sdl2::render::{ Texture, BlendMode };
use sdl2::pixels::{ Color };
use std::intrinsics::transmute;

#[derive(Clone)]
pub enum RenderOperation {
    Image(Rc<::resource::Texture>),
    Label(String, Rc<::resource::Font>, Color),
    Round(Color)
}

pub struct RenderTree {
    node: Rc<dyn NodeLike>,
    operation: RefCell<Option<RenderOperation>>,
    children: RefCell<Vec<Rc<RenderTree>>>
}

impl RenderTree {

    pub fn new(node: Rc<dyn NodeLike>) -> Rc<Self> {
        Rc::new(Self {
            node: node,
            operation: RefCell::new(None),
            children: RefCell::new(Vec::new())
        })
    }

    pub fn set_operation(&self, operation: RenderOperation) {
        self.operation.replace(Some(operation));
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

    pub fn convert_window_point_to_resolution_point(&self, point: &Point) -> Point {
        if point.x() < 0 && point.y() < 0 { return point.clone(); }
        let application = self.get_application();
        let window_size = application.window_size();
        let render_dest = self.render_canvas_dest.clone().unwrap_or(Rect::new(0, 0, window_size.width(), window_size.height()));
        if render_dest.x() > point.x() ||
            render_dest.x() + (render_dest.width() as i32) < point.x() ||
            render_dest.y() > point.y() ||
            render_dest.y() + (render_dest.height() as i32) < point.y() {
            return Point::new(-1, -1);
        }
        let normalized_point = Point::new(point.x() - render_dest.x(), point.y() - render_dest.y());
        let magni: (f32, f32) = (self.resolution_size.width() as f32 / render_dest.width() as f32, self.resolution_size.height() as f32 / render_dest.height() as f32);
        Point::new((normalized_point.x() as f32 * magni.0) as i32, (normalized_point.y() as f32 * magni.1) as i32)
    }

    pub fn set_application(&mut self, application: Rc<dyn Application>) {
        self.application = Some(application.clone());
        self.resolution_size = application.resolution_size();
        self.resolution_policy = application.resolution_policy();
        self.render_canvas_dest = Self::generate_render_canvas_dest(application);
    }

    fn get_application(&self) -> Rc<dyn Application> {
        self.application.clone().unwrap()
    }

    pub fn add_alias(&mut self, name: &str, path: &str) {
        self.resource.add_alias(name, path);
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<::resource::Texture> {
        self.resource.load_texture(path)
    }

    pub fn load_font(&mut self, option: &OneLineLabelOption) -> Rc<::resource::Font> {
        self.resource.load_font(option)
    }

    pub fn prepare_render_tree(&mut self, parent: Option<Rc<dyn NodeLike>>, node: Rc<dyn NodeLike>) {
        let id = node.id();
        if self.render_tree_nodes.get(&id).is_none() {
            let tree_node = RenderTree::new(node.clone());
            self.render_tree_nodes.insert(id.clone(), tree_node.clone());
            if let Some(p) = parent {
                self.render_tree_nodes.get(&p.id()).unwrap().push_child(tree_node.clone());
            }
        }
    }

    pub fn render_texture(&mut self, node: Rc<dyn NodeLike>, texture: Rc<::resource::Texture>) {
        let tree = self.render_tree_nodes.get(&node.id()).unwrap();
        tree.set_operation(RenderOperation::Image(texture));
    }

    pub fn render_label(&mut self, node: Rc<dyn NodeLike>, text: &str, font: Rc<::resource::Font>, color: &Color) {
        let tree = self.render_tree_nodes.get(&node.id()).unwrap();
        tree.set_operation(RenderOperation::Label(text.to_owned(), font, color.clone()));
    }

    pub fn render_round(&self, node: Rc<dyn NodeLike>, color: &Color) {
        let tree = self.render_tree_nodes.get(&node.id()).unwrap();
        tree.set_operation(RenderOperation::Round(color.clone()));
    }

    pub fn measure_label_size(&self, text: &str, font: Rc<::resource::Font>) -> Size {
        let f = self.resource.load_font_from_resource_key(font);
        let surface = f.render(text).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        Size::new(surface.width(), surface.height())
    }

    pub fn update_resolution_size(&mut self) {
        let application = self.get_application();
        let (resolution_size, resolution_policy) = (application.resolution_size(), application.resolution_policy());
        if self.resolution_size != resolution_size || self.resolution_policy != resolution_policy {
            self.resolution_size = resolution_size;
            self.resolution_policy = resolution_policy;
            self.render_canvas_dest = Self::generate_render_canvas_dest(application);
        }
    }

    fn create_sub_canvas(&self, size: Size) -> Texture<'a> {
        with_context(|c| c.create_sub_canvas(size.clone()))
    }

    fn set_alpha_blend_mode(&self, canvas: &mut Texture<'a>) {
        let ret = unsafe {
            let mode = sdl2::sys::SDL_ComposeCustomBlendMode(
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
            );
            sdl2::sys::SDL_SetTextureBlendMode(canvas.raw(), transmute(mode as u32))
        };
        if ret != 0 { panic!("合成モードの設定が失敗しました") }
    }

    fn render_inner_canvas(&mut self, render_tree: Rc<RenderTree>) -> Option<Rc<Texture<'a>>> {
        let node = render_tree.node.clone();
        if !node.get_visible() { return None; }
        if let Some(cache) = self.load_by_render_cache(node.clone()) {
            return Some(cache);
        }
        let mut sub_canvas = if let Some(operation) = &*render_tree.operation.borrow() {
            self.render_operation(node.clone(), operation)
        } else {
            self.render_children(render_tree.clone())
        };
        if node.is_additive_blend() { sub_canvas.set_blend_mode(BlendMode::Add); }
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

    fn render_operation(&mut self, node: Rc<dyn NodeLike>, operation: &RenderOperation) -> Texture<'a> {
        let texture = self.exec_operation(node.clone(), operation);
        let mut ct = self.clone_texture(&texture);
        if node.get_opacity() == 255 { return ct; }
        self.apply_alpha_mod(&mut ct, node.get_opacity())
    }

    fn exec_operation(&mut self, node: Rc<dyn NodeLike>, operation: &RenderOperation) -> Rc<Texture<'a>> {
        match operation {
            RenderOperation::Image(texture) => {
                self.resource.load_texture_from_resource_key(texture.clone())
            },
            RenderOperation::Label(text, font, color) => {
                let f = self.resource.load_font_from_resource_key(font.clone());
                let surface = f.render(text.as_str()).blended(*color).unwrap();
                let texture = with_context(|c| c.texture_creator.create_texture_from_surface(surface)).unwrap();
                Rc::new(texture)
            },
            RenderOperation::Round(color) => {
                let mut ct = self.create_sub_canvas(node.get_size());
                if color.a == 0 { return Rc::new(ct); }
                with_context(|c| &mut c.canvas).with_texture_canvas(&mut ct, |c| {
                    c.set_draw_color(color.clone());
                    let size  = node.get_size();
                    let radius = size.height() as f32 / 2.0;
                    let r = radius.powi(2);
                    for i in 0..size.width() {
                        let x = i as f32 - size.width() as f32 / 2.0;
                        let height = (r - x.powi(2)).sqrt().round();
                        c.draw_line(
                            *Point::new(i as i32, (radius - height).round() as i32),
                            *Point::new(i as i32, (radius + height).round() as i32)
                        ).unwrap();
                    }
                });
                Rc::new(ct)
            }
        }
    }

    fn render_children(&mut self, render_tree: Rc<RenderTree>) -> Texture<'a> {
        let children: Vec<(Rc<dyn NodeLike>, Option<Rc<Texture<'a>>>)> = render_tree.children.borrow().iter().map(|child| {
            (child.node.clone(), self.render_inner_canvas(child.clone()))
        }).collect();
        let mut ct = self.create_sub_canvas(render_tree.node.get_size());
        self.set_alpha_blend_mode(&mut ct);
        with_context(|c| &mut c.canvas).with_texture_canvas(&mut ct, |c| {
            for (child_node, child_texture) in children {
                if let Some(t) = child_texture {
                    let rect = child_node.get_render_rect();
                    let angle = child_node.get_rotation();
                    c.copy_ex(&t, None, Some(rect.into()), angle, None, false, false).unwrap();
                }
            }
        }).unwrap();
        if render_tree.node.get_opacity() == 255 { return ct; }
        self.apply_alpha_mod(&mut ct, render_tree.node.get_opacity())
    }

    fn clone_texture(&self, texture: &Texture<'a>) -> Texture<'a> {
        let query = texture.query();
        let mut sub_canvas = self.create_sub_canvas(Size::new(query.width, query.height));
        sub_canvas.set_blend_mode(BlendMode::None);
        with_context(|c| &mut c.canvas).with_texture_canvas(&mut sub_canvas, |c| {
            c.copy(texture, None, None).unwrap();
        }).unwrap();
        self.set_alpha_blend_mode(&mut sub_canvas);
        sub_canvas
    }

    fn apply_alpha_mod(&self, texture: &mut Texture<'a>, alpha: u8) -> Texture<'a> {
        let query = texture.query();
        let mut sub_canvas = self.create_sub_canvas(Size::new(query.width, query.height));
        texture.set_alpha_mod(alpha);
        sub_canvas.set_blend_mode(BlendMode::Blend);
        with_context(|c| &mut c.canvas).with_texture_canvas(&mut sub_canvas, |c| {
            c.copy(texture, None, None).unwrap();
        }).unwrap();
        sub_canvas
    }

    fn render_scene(&mut self, scene_id: NodeId) -> Texture<'a> {
        let render_tree = self.render_tree_nodes.get(&scene_id).cloned().unwrap();
        if let Some(texture) = self.render_inner_canvas(render_tree.clone()) {
            self.clone_texture(&texture)
        } else {
            self.create_sub_canvas(render_tree.node.get_size())
        }
    }

    pub fn render_canvas(&mut self, scene: Rc<dyn SceneLike>, prev_scene: Option<Rc<dyn SceneLike>>, transition: Rc<SceneTransition>) -> TransitionStatus {
        let scene_canvas = self.render_scene(scene.id());
        let (canvas, status) = if let Some(prev_scene) = prev_scene.clone() {
            let prev_canvas = self.render_scene(prev_scene.id());
            let t = transition.render(scene_canvas, prev_canvas);
            (t, transition.get_status())
        } else {
            (scene_canvas, TransitionStatus::Wait)
        };
        with_context(|c| {
            let can = &mut c.canvas;
            can.set_draw_color(Color::RGBA(0, 0, 0, 255));
            can.clear();
            can.copy(&canvas, None, self.render_canvas_dest.clone().map(|e| e.into())).unwrap();
            can.present();
        });
        self.render_tree_nodes = HashMap::new();
        status
    }

    pub fn destroy_render_cache(&mut self, key: &ResourceKey) {
        self.resource.destroy_render_cache(key);
    }

}
