#[derive(Clone)]
pub struct Redis {
    db: String,
}

impl Redis {
    pub fn new() -> Self {
        let db = "hello".to_string();
        Self { db }
    }
    pub fn get_db(&self) -> &String {
        &self.db
    }
}
