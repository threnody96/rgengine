use std::collections::HashMap;
use std::cell::RefCell;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::value::Value;
use ::util::unwrap;

pub struct Variable {
    map: RefCell<HashMap<String, Value>>
}

impl Variable {

    pub fn new() -> Self {
        Self { map: RefCell::new(HashMap::new()) }
    }

    pub fn get<T, K>(&self, index: K) -> T
    where T: DeserializeOwned, K: ToString
    {
        let map = self.map.borrow();
        let vindex = index.to_string();
        let v = unwrap(map.get(vindex.as_str()).ok_or(format!("unknown variable: {}", vindex)));
        unwrap(serde_json::from_value(v.clone()).map_err(|e| e.to_string()))
    }

    pub fn set<T, K>(&self, index: K, val: T)
    where T: Serialize, K: ToString
    {
        let mut map = self.map.borrow_mut();
        let vindex = index.to_string();
        map.insert(vindex, json!(val));
    }

}
