pub trait Storage {

    fn name(&self) -> String;
    fn load(&self, path: String) -> Result<Vec<u8>, String>;
    fn list(&self, dir: Option<String>) -> Result<Vec<String>, String>;
    fn save(&self, path: String, data: &Vec<u8>) -> Result<(), String>;

}
