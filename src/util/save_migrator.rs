use serde_json::{ Value };
use serde_json::map::Map;

pub trait SaveMigrator {

    fn migrate(&self, version: &str, variables: &mut Map<String, Value>) -> String;

}