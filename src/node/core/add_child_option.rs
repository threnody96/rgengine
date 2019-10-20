pub struct AddChildOption {
    pub z_index: i32,
    pub tag: Option<String>
}

impl Default for AddChildOption {

    fn default() -> Self {
        Self {
            z_index: 0,
            tag: None
        }
    }

}
