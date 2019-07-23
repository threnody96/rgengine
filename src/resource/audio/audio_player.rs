use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::borrow::Borrow;
use ggez::Context;
use ggez::audio::{ Source, SoundData, SoundSource };
use ::util::unwrap;
use ::resource::Resource;

pub struct AudioPlayer {
    resource: Rc<Resource>,
    channel: RefCell<HashMap<String, Source>>
}

impl AudioPlayer {

    pub fn new(resource: Rc<Resource>) -> Self {
        Self {
            resource: resource,
            channel: RefCell::new(HashMap::new())
        }
    }

    pub fn play<T>(&self, ctx: &mut Context, channel_key: T, name: &str, path: &str)
    where T: ToString + Clone
    {
        let data = self.resource().load_sound(ctx, name, path);
        self.play_data(ctx, channel_key, data);
    }

    pub fn play_data<T>(&self, ctx: &mut Context, channel_key: T, data: Rc<SoundData>)
    where T: ToString + Clone
    {
        let mut s = self.source(ctx, data);
        self.stop(channel_key.clone());
        unwrap(s.play());
        let mut channel = self.channel.borrow_mut();
        channel.insert(channel_key.to_string(), s);
    }

    pub fn play_once(&self, ctx: &mut Context, name: &str, path: &str) {
        let data = self.resource().load_sound(ctx, name, path);
        self.play_data_once(ctx, data);
    }

    pub fn play_data_once(&self, ctx: &mut Context, data: Rc<SoundData>) {
        let mut s = self.source(ctx, data);
        unwrap(s.play_detached());
    }

    pub fn stop<T>(&self, channel_key: T)
    where T: ToString + Clone
    {
        let mut channel = self.channel.borrow_mut();
        match channel.remove(channel_key.to_string().as_str()) {
            None => {},
            Some(mut s) => { s.stop(); }
        }
    }

    pub fn pause<T>(&self, channel_key: T)
    where T: ToString + Clone
    {
        let channel = self.channel.borrow();
        match channel.get(channel_key.to_string().as_str()) {
            None => {},
            Some(s) => { s.pause(); }
        }
    }

    pub fn resume<T>(&self, channel_key: T)
    where T: ToString + Clone
    {
        let channel = self.channel.borrow();
        match channel.get(channel_key.to_string().as_str()) {
            None => {},
            Some(s) => { s.resume(); }
        }
    }

    fn source(&self, ctx: &mut Context, data: Rc<SoundData>) -> Source {
        let borrowed: &SoundData = data.borrow();
        unwrap(Source::from_data(ctx, borrowed.clone()))
    }

    fn resource(&self) -> &Resource {
        self.resource.borrow()
    }

}
