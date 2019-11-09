use std::rc::Rc;
use std::collections::HashMap;
use ::director::resource::ResourceDirector;
use ::resource::{SE, ResourceKey, ResourceType };
use ::util::parameter::{MusicOption};
use sdl2::mixer::{ Music, Chunk, Channel };
use uuid::Uuid;

pub struct SoundDirector<'a> {
    resource: ResourceDirector<'a>,
    ses: HashMap<String, Channel>
}

impl <'a> SoundDirector<'a> {

    pub fn new() -> Self {
        Self {
            resource: ResourceDirector::new(),
            ses: HashMap::new()
        }
    }

    pub fn add_alias(&mut self, name: &str, path: &str) {
        self.resource.add_alias(name, path);
    }

    pub fn play_music(&mut self, path: &str, option: MusicOption) {
        let m = self.resource.load_music(path);
        let loops = option.loops;
        let fade_in = option.fade_in.clone().unwrap_or(0);
        let position = option.position.clone().unwrap_or(0.0);
        m.fade_in_from_pos(loops, fade_in, position).unwrap();
    }

    pub fn stop_music(&self, fade_out: i32) {
        if fade_out <= 0 {
            Music::halt();
        } else {
            Music::fade_out(fade_out);
        }
    }

    pub fn play_se(&mut self, path: &str) -> Rc<SE> {
        let channel_id = self.generate_new_channel();
        let se = self.resource.load_se(path);
        let channel = self.ses.get(&channel_id).unwrap();
        channel.play(&se, 0);
        Rc::new(SE::new(
            ResourceKey::new(path, ResourceType::SE),
            channel_id
        ))
    }

    pub fn stop_se(&self, se: Rc<SE>) {
        if let Some(channel) = self.ses.get(&se.channel()) {
            channel.halt();
        }
    }

    pub fn stop_all_se(&self) {
        for (channel, se) in &self.ses {
            se.halt();
        }
    }

    pub fn clean_se(&mut self, seed: usize) {
        let key = {
            let channels: Vec<&String> = self.ses.keys().collect();
            if channels.len() == 0 { return; }
            let index = seed % channels.len();
            channels.get(index).unwrap().to_string()
        };
        let se = self.ses.get(&key).unwrap();
        if !se.is_paused() && !se.is_playing() {
            self.ses.remove(&key);
        }
    }

    fn generate_new_channel(&mut self) -> String {
        let channel_id = self.generate_channel_id();
        self.ses.insert(channel_id.clone(), Channel(-1));
        channel_id
    }

    fn generate_channel_id(&self) -> String {
        let mut id = "".to_owned();
        loop {
            id = Uuid::new_v4().to_string();
            if self.ses.get(&id).is_none() { break; }
        }
        id
    }

}