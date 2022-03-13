use crate::core::{failable::Failable, logs};
use std::{collections::HashMap, rc::Rc};

pub struct Cache<T> {
    log_tag: String,
    store: HashMap<String, Rc<T>>,
    factory: fn(key: &str) -> Failable<T>,
}

impl<T> Cache<T> {
    pub fn new(log_tag: &str, factory: fn(key: &str) -> Failable<T>) -> Self {
        Cache {
            log_tag: log_tag.to_owned(),
            store: HashMap::new(),
            factory: factory,
        }
    }

    pub fn get(&mut self, key: &str) -> Failable<Rc<T>> {
        if !self.store.contains_key(key) {
            logs::out(&format!("cache: {}", &self.log_tag), &format!("Creating: '{}'", key));
            self.store.insert(key.to_owned(), Rc::new((self.factory)(key)?));
        }

        Ok(self.store.get(key).ok_or("Caching error.")?.clone())
    }
}
