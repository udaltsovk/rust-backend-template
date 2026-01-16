pub mod error;

pub struct Namespace {
    path: String,
}

impl Namespace {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            path: name.to_string(),
        }
    }

    #[must_use]
    pub fn nest(&self, name: &str) -> Self {
        Self {
            path: self.key(name),
        }
    }

    #[must_use]
    pub fn key(&self, name: &str) -> String {
        format!("{}::{name}", self.path,)
    }
}
