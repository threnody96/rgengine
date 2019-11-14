mod application;
mod node;
mod resource;
mod render;
mod input;
mod sound;
mod variable;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use ::application::{ Application };
use ::util::{ SaveMigrator };
use ::util::parameter::{ Size, InputCode, InputInfo, Point, MusicOption };
use ::node::{ Node, NodeLike, NodeDelegate, NodeId };
use ::node::scene::{ SceneLike };
use ::node::scene::transition::{ SceneTransition, TransitionStatus };
use ::node::label::{ LabelOption, OneLineLabelOption };
use ::resource::{ Texture, Font, ResourceKey, SE };
use self::application::ApplicationDirector;
use self::node::NodeDirector;
use self::render::RenderDirector;
use self::input::InputDirector;
use self::sound::SoundDirector;
use self::variable::VariableDirector;
use self::resource::ResourceDirector;
use sdl2::{ EventPump };
use sdl2::pixels::{ Color };
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::{ Value };
use rand::distributions::{ Standard, Distribution };

pub struct Director<'a> {
    application: RefCell<ApplicationDirector>,
    node: RefCell<NodeDirector>,
    render: RefCell<RenderDirector<'a>>,
    input: RefCell<InputDirector>,
    sound: RefCell<SoundDirector<'a>>,
    variable: RefCell<VariableDirector>,
    resource: RefCell<ResourceDirector<'a>>
}

impl <'a> Director<'a> {

    pub fn new() -> Self {
        Self {
            application: RefCell::new(ApplicationDirector::new()),
            node: RefCell::new(NodeDirector::new()),
            render: RefCell::new(RenderDirector::new()),
            input: RefCell::new(InputDirector::new()),
            sound: RefCell::new(SoundDirector::new()),
            variable: RefCell::new(VariableDirector::new()),
            resource: RefCell::new(ResourceDirector::new())
        }
    }

    pub fn window_size(&self) -> Size {
        self.application.borrow().window_size()
    }

    pub fn get_resolution_size(&self) -> Size {
        self.application.borrow().get_resolution_size()
    }

    pub fn rand<T>(&self) -> T where Standard: Distribution<T> {
        self.application.borrow_mut().rand()
    }

    pub fn is_continuing(&self) -> bool {
        self.application.borrow().is_continuing()
    }

    pub fn set_continuing(&self, continuing: bool) {
        self.application.borrow_mut().set_continuing(continuing);
    }

    pub fn get_scene(&self) -> Rc<dyn SceneLike> {
        self.application.borrow().get_scene()
    }

    pub fn get_prev_scene(&self) -> Option<Rc<dyn SceneLike>> {
        self.application.borrow().get_prev_scene()
    }

    pub fn get_scene_transition(&self) -> Rc<SceneTransition> {
        self.application.borrow().get_scene_transition()
    }

    pub fn replace_scene<T>(&self, scene: Rc<dyn SceneLike>, transition: T)
    where T: Into<Rc<SceneTransition>>
    {
        self.application.borrow_mut().replace_scene(scene, transition.into());
    }

    pub fn push_scene<T>(&self, scene: Rc<dyn SceneLike>, transition: T)
        where T: Into<Rc<SceneTransition>>
    {
        self.application.borrow_mut().push_scene(scene, transition.into());
    }

    pub fn pop_scene<T>(&self, transition: T)
        where T: Into<Rc<SceneTransition>>
    {
        self.application.borrow_mut().pop_scene(transition.into());
    }

    pub fn destroy_prev_scene(&self) {
        self.application.borrow_mut().destroy_prev_scene();
    }

    pub fn set_application(&self, application: Rc<dyn Application>) {
        self.application.borrow_mut().set_application(application.clone());
        self.render.borrow_mut().set_application(application.clone());
        self.variable.borrow_mut().set_application(application.clone());
    }

    pub fn set_current_fps(&self, fps: usize) {
        self.application.borrow_mut().set_current_fps(fps);
    }

    pub fn default_label_option(&self) -> Option<LabelOption> {
        self.application.borrow().default_label_option()
    }

    pub fn add_label_option_alias<A, B>(&self, name: A, option: B, default: bool)
    where A: Into<String>, B: Into<LabelOption>
    {
        let n = name.into();
        let o = option.into();
        self.application.borrow_mut().add_label_option_alias(&n, &o, default);
    }

    pub fn get_label_option<A>(&self, name: A) -> Option<LabelOption>
    where A: Into<String>
    {
        let n = name.into();
        self.application.borrow().get_label_option(&n)
    }

    pub fn current_fps(&self) -> usize {
        self.application.borrow().current_fps()
    }

    pub fn register_node<T>(&self, node: Rc<Node<T>>) where T: NodeDelegate + Any {
        self.node.borrow_mut().register_node(node);
    }

    pub fn get_node<T>(&self, id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
        self.node.borrow().get_node(id)
    }

    pub fn get_nodelike(&self, id: &NodeId) -> Rc<dyn NodeLike> {
        self.node.borrow().get_nodelike(id)
    }

    pub fn destroy_node(&self, id: &NodeId) {
        self.node.borrow_mut().destroy(id);
    }

    pub fn measure_label_size(&self, text: &str, font: Rc<Font>) -> Size {
        self.render.borrow().measure_label_size(text, font)
    }

    pub fn add_alias<A>(&self, name: A, path: &str) where A: Into<String> {
        let n = name.into();
        self.render.borrow_mut().add_alias(&n, path);
        self.sound.borrow_mut().add_alias(&n, path);
    }

    pub fn load_plain_data(&self, path: &str) -> Rc<Vec<u8>> {
        self.resource.borrow_mut().load_plain_data(path)
    }

    pub fn load_string(&self, path: &str) -> Rc<String> {
        self.resource.borrow_mut().load_string(path)
    }

    pub fn load_json(&self, path: &str) -> Rc<Value> {
        self.resource.borrow_mut().load_json(path)
    }

    pub fn load_texture(&self, path: &str) -> Rc<Texture> {
        self.render.borrow_mut().load_texture(path)
    }

    pub fn load_font(&self, option: &OneLineLabelOption) -> Rc<Font> {
        self.render.borrow_mut().load_font(option)
    }

    pub fn prepare_render_tree(&self, node: Rc<dyn NodeLike>) {
        self.render.borrow_mut().prepare_render_tree(node);
    }

    pub fn render_texture(&self, node: Rc<dyn NodeLike>, texture: Rc<Texture>) {
        self.render.borrow_mut().render_texture(node, texture);
    }

    pub fn render_label(&self, node: Rc<dyn NodeLike>, text: &str, font: Rc<Font>, color: &Color) {
        self.render.borrow_mut().render_label(node, text, font, color);
    }

    pub fn render_round(&self, node: Rc<dyn NodeLike>, color: &Color) {
        self.render.borrow_mut().render_round(node, color);
    }

    pub fn render_square(&self, node: Rc<dyn NodeLike>, color: &Color) {
        self.render.borrow_mut().render_square(node, color);
    }

    pub fn update_resolution_size(&self) {
        self.render.borrow_mut().update_resolution_size();
    }

    pub fn render_canvas(&self, scene: Rc<dyn SceneLike>, prev_scene: Option<Rc<dyn SceneLike>>, transition: Rc<SceneTransition>) -> TransitionStatus {
        self.render.borrow_mut().render_canvas(scene, prev_scene, transition)
    }

    pub fn destroy_render_cache(&self, key: &ResourceKey) {
        self.render.borrow_mut().destroy_render_cache(key);
    }

    pub fn get_mouse_position(&self) -> Point {
        let p = self.input.borrow().get_mouse_pointer();
        self.render.borrow().convert_window_point_to_resolution_point(&p)
    }

    pub fn get_input_info<A>(&self, key: A) -> InputInfo
    where A: Into<String>
    {
        let mut info = self.input.borrow().get_input_info(key);
        let p = self.render.borrow().convert_window_point_to_resolution_point(&info.mouse_position);
        info.mouse_position = p;
        info
    }

    pub fn update_input_state(&self, event_pump: &mut EventPump) {
        self.input.borrow_mut().update_state(event_pump)
    }

    pub fn add_key_code<A>(&self, key: A, code: InputCode)
    where A: Into<String>
    {
        self.input.borrow_mut().insert_key_code(key, code);
    }

    pub fn add_key_codes<A>(&self, codes: Vec<(A, InputCode)>)
    where A: Into<String>
    {
        for (key, code) in codes {
            self.add_key_code(key, code);
        }
    }

    pub fn reset_key_code<A>(&self, key: Option<A>)
    where A: Into<String>
    {
        self.input.borrow_mut().reset_key_code(key);
    }

    pub fn is_quit(&self) -> bool {
        self.input.borrow().is_quit()
    }

    pub fn reset_is_quit(&self) {
        self.input.borrow_mut().reset_is_quit();
    }

    pub fn play_music<A, B>(&self, path: A, option: B)
    where A: Into<String>, B: Into<MusicOption>
    {
        let p = path.into();
        self.sound.borrow_mut().play_music(&p, option.into());
    }

    pub fn stop_music(&self, fade_out: i32) {
        self.sound.borrow().stop_music(fade_out);
    }

    pub fn stop_se(&self, se: Rc<SE>) {
        self.sound.borrow().stop_se(se);
    }

    pub fn stop_all_se(&self) {
        self.sound.borrow().stop_all_se();
    }

    pub fn play_se<A>(&self, path: A) -> Rc<SE>
    where A: Into<String>
    {
        let p = path.into();
        self.sound.borrow_mut().play_se(&p)
    }

    pub fn clean_se(&self) {
        let seed: usize = self.rand();
        self.sound.borrow_mut().clean_se(seed);
    }

    pub fn get_variable<T, A>(&self, index: A) -> Option<T>
    where T: DeserializeOwned, A: Into<String>
    {
        let i = index.into();
        self.variable.borrow().get(&i)
    }

    pub fn put_variable<T, A>(&self, index: A, value: &T)
        where T: Serialize, A: Into<String>
    {
        let i = index.into();
        self.variable.borrow_mut().put(&i, value)
    }

    pub fn load_variable<T, M>(&self, name: T, migrator: M)
    where T: Into<String>, M: SaveMigrator
    {
        let n = name.into();
        self.variable.borrow_mut().load(&n, migrator)
    }

    pub fn save_variable<T>(&self, name: T) where T: Into<String> {
        let n = name.into();
        self.variable.borrow().save(&n);
    }

}
