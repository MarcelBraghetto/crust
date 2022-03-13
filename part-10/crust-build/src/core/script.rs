use std::{collections::HashMap, option::Option, path::PathBuf};

pub struct Script {
    pub content: String,
    pub working_dir: Option<PathBuf>,
    pub environment: HashMap<String, String>,

    _private: (),
}

impl Script {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.trim().to_owned(),
            environment: HashMap::new(),
            working_dir: Option::None,
            _private: (),
        }
    }

    pub fn environment(mut self, environment: &HashMap<String, String>) -> Self {
        self.environment = environment.clone();
        self
    }

    pub fn working_dir(mut self, working_dir: &PathBuf) -> Self {
        self.working_dir = Some(working_dir.clone());
        self
    }
}
