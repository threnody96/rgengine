use ::resource::{ ResourceKey };

pub struct SE {
    key: ResourceKey,
    channel: String
}

impl SE {

    pub fn new<A>(key: A, channel: String) -> Self
    where A: Into<ResourceKey>
    {
        Self {
            key: key.into(),
            channel: channel
        }
    }

    pub fn key(&self) -> ResourceKey {
        self.key.clone()
    }

    pub fn channel(&self) -> String {
        self.channel.clone()
    }

}

