use std::cell::RefCell;
use std::rc::Rc;
use ::application::{ Application };
use ::resource::{ FileStorage };
use ::util::{ ENCRYPT_KEY, SaveMigrator };
use serde_json::{ Value };
use serde_json::map::Map;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub struct VariableDirector {
    application: RefCell<Option<Rc<dyn Application>>>,
    variables: Map<String, Value>,
    storage: FileStorage
}

impl VariableDirector {

    pub fn new() -> Self {
        Self {
            application: RefCell::new(None),
            variables: Map::new(),
            storage: FileStorage::new("save", Some(ENCRYPT_KEY.to_owned()))
        }
    }

    pub fn set_application(&mut self, application: Rc<dyn Application>) {
        self.application.replace(Some(application));
    }

    fn get_application(&self) -> Rc<dyn Application> {
        self.application.borrow().clone().unwrap()
    }

    pub fn get<T>(&self, index: &str) -> Option<T> where T: DeserializeOwned {
        if let Some(v) = self.variables.get(index) {
            Some(serde_json::from_value(v.clone()).unwrap())
        } else {
            None
        }
    }

    pub fn put<T>(&mut self, index: &str, value: &T) where T: Serialize {
        let v = serde_json::to_value(value).unwrap();
        self.variables.insert(index.to_owned(), v);
    }

    pub fn load<T>(&mut self, name: &str, migrator: T) where T: SaveMigrator {
        let current_version = self.get_application().version();
        let save_data_bytes = self.storage.load(name).unwrap();
        let save_data = String::from_utf8(save_data_bytes).unwrap();
        let mut v: Value = serde_json::from_str(&save_data).unwrap();
        let mut version = v["version"].as_str().unwrap().to_owned();
        let variables = v["variables"].as_object_mut().unwrap();
        loop {
            version = migrator.migrate(&version, variables);
            if version == current_version { break; }
        }
        self.variables = variables.clone();
    }

    pub fn save(&self, name: &str) {
        let version = self.get_application().version();
        let mut map: Map<String, Value> = Map::new();
        map.insert("version".to_owned(), Value::from(version));
        map.insert("variables".to_owned(), serde_json::to_value(&self.variables).unwrap());
        let save_data = Value::from(map).to_string().as_bytes().to_vec();
        self.storage.save(name, &save_data).unwrap();
    }

}