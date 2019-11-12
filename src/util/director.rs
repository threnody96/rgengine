use std::rc::Rc;
use std::any::Any;
use ::director::{ Director };
use ::application::{ Application };
use ::node::{ NodeLike, Node, NodeDelegate, NodeId };
use ::node::scene::{ SceneLike };
use ::node::scene::transition::{ SceneTransition, TransitionStatus };
use ::node::label::{ LabelOption, OneLineLabelOption };
use ::resource::{ ResourceKey, SE, Font, Texture };
use ::util::{ SaveMigrator };
use ::util::parameter::{ Size, Color, Point, InputInfo, InputCode, MusicOption };
use rand::distributions::{ Standard, Distribution };
use sdl2::{ EventPump };
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::{ Value };

pub(crate) fn with_director<T, R>(callback: T) -> R where T: FnOnce(&Director) -> R {
    ::DIRECTOR.with(|d| {
        callback(d)
    })
}

pub fn window_size() -> Size {
    with_director(|d| d.window_size())
}

pub fn get_resolution_size() -> Size {
    with_director(|d| d.get_resolution_size())
}

pub fn rand<T>() -> T where Standard: Distribution<T> {
    with_director(|d| d.rand())
}

pub(crate) fn is_continuing() -> bool {
    with_director(|d| d.is_continuing())
}

pub fn set_continuing(continuing: bool) {
    with_director(|d| d.set_continuing(continuing))
}

pub(crate) fn get_scene() -> Rc<dyn SceneLike> {
    with_director(|d| d.get_scene())
}

pub(crate) fn get_prev_scene() -> Option<Rc<dyn SceneLike>> {
    with_director(|d| d.get_prev_scene())
}

pub(crate) fn get_scene_transition() -> Rc<SceneTransition> {
    with_director(|d| d.get_scene_transition())
}

pub fn replace_scene<T>(scene: Rc<dyn SceneLike>, transition: T)
    where T: Into<Rc<SceneTransition>>
{
    with_director(|d| d.replace_scene(scene, transition));
}

pub fn push_scene<T>(scene: Rc<dyn SceneLike>, transition: T)
    where T: Into<Rc<SceneTransition>>
{
    with_director(|d| d.push_scene(scene, transition));
}

pub fn pop_scene<T>(transition: T)
    where T: Into<Rc<SceneTransition>>
{
    with_director(|d| d.pop_scene(transition));
}

pub(crate) fn destroy_prev_scene() {
    with_director(|d| d.destroy_prev_scene());
}

pub(crate) fn set_application(application: Rc<dyn Application>) {
    with_director(|d| d.set_application(application));
}

pub(crate) fn set_current_fps(fps: usize) {
    with_director(|d| d.set_current_fps(fps));
}

pub fn default_label_option() -> Option<LabelOption> {
    with_director(|d| d.default_label_option())
}

pub fn add_label_option_alias<A, B>(name: A, option: B, default: bool)
    where A: Into<String>, B: Into<LabelOption>
{
    with_director(|d| d.add_label_option_alias(name, option, default));
}

pub fn get_label_option<A>(name: A) -> Option<LabelOption>
    where A: Into<String>
{
    with_director(|d| d.get_label_option(name))
}

pub fn current_fps() -> usize {
    with_director(|d| d.current_fps())
}

pub(crate) fn register_node<T>(node: Rc<Node<T>>) where T: NodeDelegate + Any {
    with_director(|d| d.register_node(node));
}

pub fn get_node<T>(id: &NodeId) -> Option<Rc<Node<T>>> where T: NodeDelegate + Any {
    with_director(|d| d.get_node(id))
}

pub fn get_nodelike(id: &NodeId) -> Rc<dyn NodeLike> {
    with_director(|d| d.get_nodelike(id))
}

pub(crate) fn destroy_node(id: &NodeId) {
    with_director(|d| d.destroy_node(id));
}

pub(crate) fn measure_label_size(text: &str, font: Rc<Font>) -> Size {
    with_director(|d| d.measure_label_size(text, font))
}

pub fn add_alias<A>(name: A, path: &str) where A: Into<String> {
    with_director(|d| d.add_alias(name, path));
}

pub fn load_plain_data(path: &str) -> Rc<Vec<u8>> {
    with_director(|d| d.load_plain_data(path))
}

pub fn load_string(path: &str) -> Rc<String> {
    with_director(|d| d.load_string(path))
}

pub fn load_json(path: &str) -> Rc<Value> {
    with_director(|d| d.load_json(path))
}

pub fn load_texture(path: &str) -> Rc<Texture> {
    with_director(|d| d.load_texture(path))
}

pub fn load_font(option: &OneLineLabelOption) -> Rc<Font> {
    with_director(|d| d.load_font(option))
}

pub(crate) fn prepare_render_tree(node: Rc<dyn NodeLike>) {
    with_director(|d| d.prepare_render_tree(node));
}

pub(crate) fn render_texture(node: Rc<dyn NodeLike>, texture: Rc<Texture>) {
    with_director(|d| d.render_texture(node, texture));
}

pub(crate) fn render_label(node: Rc<dyn NodeLike>, text: &str, font: Rc<Font>, color: &Color) {
    with_director(|d| d.render_label(node, text, font, color));
}

pub(crate) fn render_round(node: Rc<dyn NodeLike>, color: &Color) {
    with_director(|d| d.render_round(node, color));
}

pub(crate) fn update_resolution_size() {
    with_director(|d| d.update_resolution_size());
}

pub(crate) fn render_canvas(scene: Rc<dyn SceneLike>, prev_scene: Option<Rc<dyn SceneLike>>, transition: Rc<SceneTransition>) -> TransitionStatus {
    with_director(|d| d.render_canvas(scene, prev_scene, transition))
}

pub(crate) fn destroy_render_cache(key: &ResourceKey) {
    with_director(|d| d.destroy_render_cache(key));
}

pub fn get_mouse_position() -> Point {
    with_director(|d| d.get_mouse_position())
}

pub fn get_input_info<A>(key: A) -> InputInfo
    where A: Into<String>
{
    with_director(|d| d.get_input_info(key))
}

pub(crate) fn update_input_state(event_pump: &mut EventPump) {
    with_director(|d| d.update_input_state(event_pump));
}

pub fn add_key_code<A>(key: A, code: InputCode)
    where A: Into<String>
{
    with_director(|d| d.add_key_code(key, code));
}

pub fn add_key_codes<A>(codes: Vec<(A, InputCode)>)
    where A: Into<String>
{
    with_director(|d| d.add_key_codes(codes));
}

pub fn reset_key_code<A>(key: Option<A>)
    where A: Into<String>
{
    with_director(|d| d.reset_key_code(key));
}

pub(crate) fn is_quit() -> bool {
    with_director(|d| d.is_quit())
}

pub fn reset_is_quit() {
    with_director(|d| d.reset_is_quit())
}

pub fn play_music<A, B>(path: A, option: B)
    where A: Into<String>, B: Into<MusicOption>
{
    with_director(|d| d.play_music(path, option));
}

pub fn stop_music(fade_out: i32) {
    with_director(|d| d.stop_music(fade_out));
}

pub fn stop_se(se: Rc<SE>) {
    with_director(|d| d.stop_se(se));
}

pub fn stop_all_se() {
    with_director(|d| d.stop_all_se());
}

pub fn play_se<A>(path: A) -> Rc<SE>
    where A: Into<String>
{
    with_director(|d| d.play_se(path))
}

pub(crate) fn clean_se() {
    with_director(|d| d.clean_se());
}

pub fn get_variable<T, A>(index: A) -> Option<T>
    where T: DeserializeOwned, A: Into<String>
{
    with_director(|d| d.get_variable(index))
}

pub fn put_variable<T, A>(index: A, value: &T)
    where T: Serialize, A: Into<String>
{
    with_director(|d| d.put_variable(index, value));
}

pub fn load_variable<T, M>(name: T, migrator: M)
    where T: Into<String>, M: SaveMigrator
{
    with_director(|d| d.load_variable(name, migrator));
}

pub fn save_variable<T>(name: T) where T: Into<String> {
    with_director(|d| d.save_variable(name));
}
