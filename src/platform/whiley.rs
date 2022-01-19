use crate::config::{Config,Key,Error};
use crate::platform;

/// Default path for whiley source files.
static SOURCE_DEFAULT : &'static str = "src";
/// Default path for whiley binary files.
static TARGET_DEFAULT : &'static str = "bin";

static BUILD_WHILEY_SOURCE : Key = Key::new(&["build","whiley","source"]);
static BUILD_WHILEY_TARGET : Key = Key::new(&["build","whiley","target"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wyc:0.10.1",
];

pub struct WhileyPlatform {
    source: String,
    target: String
}

impl platform::JavaInstance for WhileyPlatform {
    fn name(&self) -> &'static str {
        "whiley"
    }
    fn dependencies(&self) -> &'static [&'static str] {
	MAVEN_DEPS
    }
    fn arguments(&self) -> Vec<String> {
	vec!["wyc.Main".to_string()]
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config) -> Result<platform::Instance,Error> {
	// Extract configuration (if any)
	let source = config.get_string(&BUILD_WHILEY_SOURCE).unwrap_or(SOURCE_DEFAULT.to_string());
	let target = config.get_string(&BUILD_WHILEY_TARGET).unwrap_or(TARGET_DEFAULT.to_string());	
	
	// Construct new instance on the heap
	let instance = Box::new(WhileyPlatform{source,target});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
