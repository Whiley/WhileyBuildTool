use toml::Value;
use crate::platform;

// ========================================================================
// Platform
// ========================================================================

pub struct WhileyPlatform {

}

impl platform::JavaInstance for WhileyPlatform {
    fn name(&self) -> &'static str {
        "whiley"
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Value) -> platform::Instance {
	// Construct new instance on the heap
	let instance = Box::new(WhileyPlatform{});
	// Return generic instance
	platform::Instance::Java(instance)
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
