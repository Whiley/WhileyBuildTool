pub mod whiley;

use std::collections::HashMap;
use crate::config::{Config,Error};

// ============================================================
// Instance
// ============================================================

/// A platform instance provides a generic mechanism for applying a
/// given compiler phase to the current state.  For example, compiling
/// Whiley files into WyIL files is one stage in the compiler.
/// Likewise, another stage is compiling WyIL files into JavaScript
/// files, etc.
///
/// The reason for using such a generic mechanism here is that it
/// should make it easier to add additional stages (e.g. backends for
/// other targets).
pub enum Instance {
    Java(Box<dyn JavaInstance>),
    Rust(Box<dyn RustInstance>)
}

/// Represents a platform implemented in Java.
pub trait JavaInstance {
    /// Get the name of this platform.
    fn name(&self) -> &'static str;
    /// Determine necessary Maven dependencies required for running
    /// this instance.
    fn dependencies(&self) -> &[&str];   
    /// Determine the command-line arguments which should be passed to
    /// Java.  This includes identifying the main class.
    fn arguments(&self) -> Vec<String>;
}

/// Represents a platform implemented in Rust.
pub trait RustInstance {
    /// Get the name of this platform.
    fn name(&self) -> &'static str;
}

// ============================================================
// Descriptor
// ============================================================

/// A mechanism for programmatically constructing a platform.
pub trait Descriptor {
    /// Apply this descriptor to a given TOML configuration, thereby
    /// allowing customisation of the platform instantiation.    
    fn apply<'a>(&self, config: &'a Config)->Result<Instance,Error>;
}

// ============================================================
// Descriptor
// ============================================================

/// A simple mechanism for recording the set of available platforms
/// which can be instantiated during a build.
pub struct Registry<'a> {
    registry: HashMap<&'a str, &'a dyn Descriptor>
}

impl<'a> Registry<'a> {
    /// Construct an initially empty registry.
    pub fn new() -> Registry<'a> {
        Registry{registry: HashMap::new()}
    }

    /// Register a new platform descriptor with this registry.  The
    /// descriptor is used to construct a platform instance based on
    /// the provided build configuration.    
    pub fn register(&mut self, name: &'a str, desc : &'a dyn Descriptor) {
        self.registry.insert(name,desc);
    }
    
    /// Get the desciptor associated with a given platform.
    pub fn get(&self, name: &str) -> Option<&'a dyn Descriptor> {
        match self.registry.get(name) {
            None => None,
            Some(&v) => Some(v)
        }
    }
}

