use std::error::Error;
use std::path::{Path,PathBuf};
use glob::glob;
use crate::config;
use crate::config::{Config,Key};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::platform;
use crate::platform::{PluginError,whiley};
pub static STANDARD_DEFAULT : &'static str = "ES6";
static BUILD_JAVASCRIPT_TARGET : Key = Key::new(&["build","js","target"]);
static BUILD_JAVASCRIPT_STANDARD : Key = Key::new(&["build","js","standard"]);
static BUILD_JAVASCRIPT_INCLUDES : Key = Key::new(&["build","js","includes"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    whiley::MAVEN_DEPS[0], // jmodelgen
    whiley::MAVEN_DEPS[1], // wyc
    "org.whiley:wyjs:0.10.3",
];

pub struct JavaScriptPlatform {
    name: String,
    source: String,
    target: String,
    standard: String,
    includes: Vec<String>
}

impl JavaScriptPlatform {
    fn match_includes(&self) -> Vec<String> {
	let mut matches = Vec::new();
        matches.push(self.name.clone());
        matches
    }
    //
    fn match_natives(&self) -> Vec<String> {
	let mut matches = Vec::new();
	//
	for i in &self.includes {
	    for entry in glob(&i).expect("invalid pattern for key \"build.js.includes\"") {
		match entry {
                    Ok(path) => {
			matches.push(path.to_str().unwrap().to_string());
                    }
                    Err(e) => println!("{:?}", e)
		}
            }
	}
	//
        matches
    }

    // Determine the fully qualified path of the target file.
    fn target_path(&self) -> PathBuf {
	let mut bin = PathBuf::from(&self.target);
	let mut name = self.name.clone();
	name.push_str(".js");
	bin.push(&name);
	bin
    }
}

impl platform::JavaInstance for JavaScriptPlatform {
    fn name(&self) -> &'static str {
        "js"
    }
    fn dependencies(&self) -> &'static [&'static str] {
	MAVEN_DEPS
    }
    fn arguments(&self) -> Vec<String> {
        let mut args = Vec::new();
        // Class to invoke
        args.push("wyjs.Main".to_string());
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
        target.push_str("--jsdir=");
        target.push_str(self.target.as_str());
        args.push(target);
	//
	args.push("-s".to_string());
	args.push(self.standard.clone());
        //
        args.append(&mut self.match_includes());
        //
        args
    }
    fn manifest(&self) -> Vec<build::Artifact> {
	let mut artifacts = Vec::new();
	// Register binary folder (if applicable)
	if self.target != whiley::TARGET_DEFAULT {
	    artifacts.push(Artifact::BinaryFolder(PathBuf::from(&self.target)));
	}
	// Register the binary artifact
	let bin = self.target_path();
	artifacts.push(Artifact::BinaryFile(bin));
	// Register any supplementary files
	for s in self.match_natives() {
	    artifacts.push(Artifact::SourceFile(PathBuf::from(&s)));
	}
	//
	artifacts
    }
    fn process(&self, output: &str) -> Result<Vec<build::Marker>,Box<dyn Error>> {
	if output.len() > 0 {
	    // The only way to get here should be through an internal failure.
	    Err(Box::new(PluginError{name:"wyjs".to_string(),message: output.to_string()}))
	} else {
	    Ok(Vec::new())
	}
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config, _: &Path) -> Result<platform::Instance,config::Error> {
	// Extract configuration (if any)
        let name = config.get_string(&PACKAGE_NAME)?;
	let source = config.get_string(&whiley::BUILD_WHILEY_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	let target = config.get_string(&BUILD_JAVASCRIPT_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	let standard = config.get_string(&BUILD_JAVASCRIPT_STANDARD).unwrap_or(STANDARD_DEFAULT.to_string());
	let includes = config.get_string_array(&BUILD_JAVASCRIPT_INCLUDES).unwrap_or(Vec::new());
	// Construct new instance on the heap
	let instance = Box::new(JavaScriptPlatform{name,source,target,standard,includes});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
