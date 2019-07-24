use std::collections::HashMap;
use std::cell::RefCell;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::value::Value;
use ::util::unwrap;

pub struct Variable {
    map: RefCell<HashMap<u64, Value>>
}

impl Variable {

    pub fn new() -> Self {
        Self { map: RefCell::new(HashMap::new()) }
    }

    pub fn get<T>(&self, index: u64) -> T
    where T: DeserializeOwned
    {
        let map = self.map.borrow();
        let v = unwrap(map.get(&index).ok_or(format!("unknown variable: {}", index)));
        unwrap(serde_json::from_value(v.clone()).map_err(|e| e.to_string()))
    }

    pub fn set<T>(&self, index: u64, val: T)
    where T: Serialize
    {
        let mut map = self.map.borrow_mut();
        map.insert(index, json!(val));
    }

}
