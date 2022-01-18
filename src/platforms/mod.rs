pub mod whiley;

use std::collections::HashMap;

pub trait Platform {
    fn name(&self) -> &'static str;
}

pub struct PlatformRegistry<'a> {
    registry: HashMap<&'a str, &'a dyn Platform>
}

impl<'a> PlatformRegistry<'a> {
    pub fn new() -> PlatformRegistry<'a> {
        PlatformRegistry{registry: HashMap::new()}
    }

    pub fn register(&mut self, p : &'a dyn Platform) {
        self.registry.insert(p.name(),p);
    }

    pub fn get(&self, name: &str) -> Option<&'a dyn Platform> {
        match self.registry.get(name) {
            None => None,
            Some(&v) => Some(v)
        }
    }
}

