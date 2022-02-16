use std::path::{Path,PathBuf};
use log::{error};
use crate::config::{Config,Key,Error};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::platform;
use crate::platform::whiley;

static BUILD_BOOGIE_TARGET : Key = Key::new(&["build","boogie","target"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wyc:0.10.4",    
    "org.whiley:wyboogie:0.4.0",
];

pub struct BoogiePlatform {
    name: String,
    source: String,
    target: String
}

impl BoogiePlatform {
    /// Match all whiley files to be compiled for this package.
    fn match_includes(&self) -> Vec<String> {
        let mut files = Vec::new();
        files.push(self.name.clone());
        files
    }
    // Determine the fully qualified path of the target file.
    fn target_path(&self) -> PathBuf {
	let mut bin = PathBuf::from(&self.target);
	let mut name = self.name.clone();
	name.push_str(".bpl");	
	bin.push(&name);
	bin
    }
}

impl platform::JavaInstance for BoogiePlatform {
    fn name(&self) -> &'static str {
        "boogie"
    }
    fn dependencies(&self) -> &'static [&'static str] {
	MAVEN_DEPS
    }
    fn arguments(&self) -> Vec<String> {
        let mut args = Vec::new();
        // Class to invoke
        args.push("wyboogie.Main".to_string());
        // Target name
        args.push("-o".to_string());
        args.push(self.name.clone());
        // Whiley bin dir
        let mut source = String::new();
        source.push_str("--wyildir=");
        source.push_str(self.source.as_str());
        args.push(source);
        // JavaScript bin dir
        let mut target = String::new();
        target.push_str("--bpldir=");
        target.push_str(self.target.as_str());
        args.push(target);
        //
        args.append(&mut self.match_includes());
        //
        args
    }
    fn manifest(&self) -> Vec<build::Artifact> {
	let mut artifacts = Vec::new();
	// Register the binary artifact
	let bin = self.target_path();
	artifacts.push(Artifact::Binary(bin));	
	//
	artifacts
    }
    fn process(&self, output: &str) -> Vec<build::Marker> {
	if output.len() > 0 {
            // In principle its possible to get here if there is some
            // kind of internal failure.  However, it remains an issue
            // at this stage as to how to process it.
	    error!("wyboogie: {}",output);
            panic!("deadcode reached!");
        }
        Vec::new()
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config, whileyhome: &Path) -> Result<platform::Instance,Error> {
	// Extract configuration (if any)
        let name = config.get_string(&PACKAGE_NAME)?;
	let source = config.get_string(&whiley::BUILD_WHILEY_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	let target = config.get_string(&BUILD_BOOGIE_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	// Construct new instance on the heap
	let instance = Box::new(BoogiePlatform{name,source,target});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
