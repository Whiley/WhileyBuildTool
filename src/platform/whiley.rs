use std::path::PathBuf;
use glob::glob;
use crate::config::{Config,Key,Error};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::platform;

/// Default path for whiley source files.
static SOURCE_DEFAULT : &'static str = "src";
/// Default path for whiley binary files.
static TARGET_DEFAULT : &'static str = "bin";
/// Default set of includes for whiley files
static INCLUDES_DEFAULT : &'static str = "**/*.whiley";

static BUILD_WHILEY_SOURCE : Key = Key::new(&["build","whiley","source"]);
static BUILD_WHILEY_TARGET : Key = Key::new(&["build","whiley","target"]);
static BUILD_WHILEY_INCLUDES : Key = Key::new(&["build","whiley","includes"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wyc:0.10.3",
];

pub struct WhileyPlatform {
    name: String,
    source: String,
    target: String,
    includes: String
}

impl WhileyPlatform {
    /// Match all whiley files to be compiled for this package.
    fn match_includes(&self) -> Vec<String> {
        // TODO: this is all rather ugly if you ask me.	
	let mut matches = Vec::new();
        let mut includes = String::new();
        includes.push_str(self.source.as_str());
        includes.push_str("/");
        includes.push_str(self.includes.as_str());
        //
        for entry in glob(&includes).expect("invalid pattern for key \"build.whiley.includes\"") {
            match entry {
                Ok(path) => {
                    let f = path.into_os_string().into_string().unwrap();
                    let n = self.source.len()+1;
                    matches.push(f[n..].to_string());
                }
                Err(e) => println!("{:?}", e)
            }
        }
	// Done
	matches
    }
    // Determine the fully qualified path of the target file.
    fn target_path(&self) -> PathBuf {
	let mut bin = PathBuf::from(&self.target);
	let mut name = self.name.clone();
	name.push_str(".wyil");	
	bin.push(&name);
	bin
    }
}

impl platform::JavaInstance for WhileyPlatform {
    fn name(&self) -> &'static str {
        "whiley"
    }
    fn dependencies(&self) -> &'static [&'static str] {
	MAVEN_DEPS
    }
    fn arguments(&self) -> Vec<String> {
        let mut args = Vec::new();
        // Class to invoke
        args.push("wyc.Main".to_string());
	// Brief mode
	args.push("-b".to_string());
        // Target name
        args.push("-o".to_string());
        args.push(self.name.clone());
        // Whiley source dir
        let mut source = String::new();
        source.push_str("--whileydir=");
        source.push_str(self.source.as_str());
        args.push(source);
        // Whiley bin dir
        let mut target = String::new();
        target.push_str("--wyildir=");
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
	for i in self.match_includes() {
	    let mut p = PathBuf::from(&self.source);
	    p.push(i);
	    artifacts.push(Artifact::Source(p));
	}
	//
	artifacts
    }
    fn process(&self, output: &str) -> Vec<build::Marker> {
	let mut markers = Vec::new();
	// Process each line of output
	for line in output.lines() {
	    // Split line into components
	    let split : Vec<&str> = line.split(":").collect();
	    // Parse components
	    let kind = build::Kind::SyntaxError;
	    let mut path = PathBuf::from(&self.source);
	    path.push(split[0]);
	    let start = split[1].parse().unwrap();
	    let end = split[2].parse().unwrap();
	    let message = split[3].to_string();
	    // Done
	    markers.push(build::Marker::new(kind,path,start,end,message));
	}
	// Done
	markers
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config) -> Result<platform::Instance,Error> {
	// Extract configuration (if any)
        let name = config.get_string(&PACKAGE_NAME)?;
	let source = config.get_string(&BUILD_WHILEY_SOURCE).unwrap_or(SOURCE_DEFAULT.to_string());
	let target = config.get_string(&BUILD_WHILEY_TARGET).unwrap_or(TARGET_DEFAULT.to_string());
	let includes = config.get_string(&BUILD_WHILEY_INCLUDES).unwrap_or(INCLUDES_DEFAULT.to_string());
	// Construct new instance on the heap
	let instance = Box::new(WhileyPlatform{name,source,target,includes});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
