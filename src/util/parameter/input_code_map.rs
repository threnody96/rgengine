use ::util::parameter::{ InputCode };
use std::collections::HashMap;

pub struct InputCodeMap {
    map: HashMap<String, Vec<InputCode>>
}

impl InputCodeMap {

    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub(crate) fn convert_key<A>(&self, key: A) -> Vec<InputCode>
        where A: Into<String>
    {
        self.map.get(&key.into()).cloned().unwrap_or(Vec::new())
    }

    pub fn insert<A>(&mut self, key: A, code: InputCode)
        where A: Into<String>
    {
        let k = key.into();
        if self.map.get(&k).is_none() {
            self.map.insert(k.clone(), Vec::new());
        }
        self.map.get_mut(&k).unwrap().push(code);
    }

    pub fn reset<A>(&mut self, key: Option<A>)
        where A: Into<String>
    {
        match key {
            Some(k) => {
                self.map.insert(k.into(), Vec::new());
            },
            None => {
                self.map = HashMap::new();
            }
        }
    }

}

