use std::error::Error;
use std::path::{Path,PathBuf};
use glob::glob;
use crate::config;
use crate::config::{Config,Key};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::jvm;
use crate::platform;
use crate::platform::{PluginError};

/// Default setting for whether building library or binary.
pub static LIBRARY_DEFAULT : bool = false;
/// Default path for whiley source files.
pub static SOURCE_DEFAULT : &'static str = "src";
/// Default path for whiley binary files.
pub static TARGET_DEFAULT : &'static str = "bin";
/// Default set of includes for whiley files
pub static INCLUDES_DEFAULT : &'static str = "**/*.whiley";

pub static DEPENDENCIES : Key = Key::new(&["dependencies"]);
pub static BUILD_WHILEY_SOURCE : Key = Key::new(&["build","whiley","source"]);
pub static BUILD_WHILEY_TARGET : Key = Key::new(&["build","whiley","target"]);
pub static BUILD_WHILEY_INCLUDES : Key = Key::new(&["build","whiley","includes"]);
pub static BUILD_WHILEY_LIBRARY : Key = Key::new(&["build","whiley","library"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
pub static MAVEN_DEPS : &'static [&str] = &[
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wyc:0.10.6",
];

pub struct WhileyPlatform {
    name: String,
    linking: bool,
    source: String,
    target: String,
    includes: String,
    whileypath: Vec<String>
}

impl WhileyPlatform {
    /// Match all whiley files to be compiled for this package.
    fn match_includes(&self) -> Vec<String> {
        // TODO: this is all rather ugly if you ask me.
	let mut matches = Vec::new();
        let mut includes = PathBuf::new();
	includes.push(self.source.as_str());
        includes.push(self.includes.as_str());
	let mut sincludes = includes.to_str().unwrap();
        //
        for entry in glob(&sincludes).expect("invalid pattern for key \"build.whiley.includes\"") {
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
        args.push("wyc.Compiler".to_string());
	// Brief mode
	args.push("-b".to_string());
        // Target name
        args.push("-o".to_string());
        args.push(self.name.clone());
	//
	if self.linking {
	    // Enable linking
	    args.push("--linking".to_string());
	}
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
        // Whiley path
        let mut whileypath = String::new();
        if self.whileypath.len() > 0 {
            whileypath.push_str("--whileypath=");
            whileypath.push_str(self.whileypath.get(0).unwrap());
            for e in &self.whileypath[1..] {
                whileypath.push_str(jvm::classpath_sep());
                whileypath.push_str(e);
            }
	    args.push(whileypath);
        }
        //
        args.append(&mut self.match_includes());
        //
        args
    }
    fn manifest(&self) -> Vec<build::Artifact> {
	let mut artifacts = Vec::new();
	// Register the binary artifact
	let bin = self.target_path();
	artifacts.push(Artifact::BinaryFolder(PathBuf::from(&self.target)));
	artifacts.push(Artifact::BinaryFile(bin));
	artifacts.push(Artifact::SourceFolder(PathBuf::from(&self.source)));
	for i in self.match_includes() {
	    let mut p = PathBuf::from(&self.source);
	    p.push(i);
	    artifacts.push(Artifact::SourceFile(p));
	}
	//
	artifacts
    }
    fn process(&self, output: &str) -> Result<Vec<build::Marker>,Box<dyn Error>> {
	match parse_output(&self.source,output) {
	    Some(markers) => Ok(markers),
	    None => {
		Err(Box::new(PluginError{name:"wyc".to_string(),message: output.to_string()}))
	    }
	}
    }
}

pub fn parse_output(source: &str, output: &str) -> Option<Vec<build::Marker>> {
    let mut markers = Vec::new();
    // Process each line of output
    for line in output.lines() {
	// Split line into components
	let split : Vec<&str> = line.split(":").collect();
	if split.len() != 4 {
	    return None;
	}
	// Parse components
	let kind = build::Kind::SyntaxError;
	let mut path = PathBuf::from(source);
	path.push(split[0]);
	let start = split[1].parse();
	let end = split[2].parse();
	if start.is_err() || end.is_err() {
	    return None;
	}
	let message = split[3].to_string();
	// Done
	markers.push(build::Marker::new(kind,path,start.unwrap(),end.unwrap(),message));
    }
    // Done
    Some(markers)
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

const TMP : &'static str = "dependencies";

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config, whileyhome: &Path) -> Result<platform::Instance,config::Error> {
	// Extract configuration (if any)
        let name = config.get_string(&PACKAGE_NAME)?;
	let linking = !config.get_bool(&BUILD_WHILEY_LIBRARY).unwrap_or(LIBRARY_DEFAULT);
	let source = config.get_string(&BUILD_WHILEY_SOURCE).unwrap_or(SOURCE_DEFAULT.to_string());
	let target = config.get_string(&BUILD_WHILEY_TARGET).unwrap_or(TARGET_DEFAULT.to_string());
	let includes = config.get_string(&BUILD_WHILEY_INCLUDES).unwrap_or(INCLUDES_DEFAULT.to_string());
        // Construct whileypath?
        let mut whileypath = Vec::new();
	// FIXME: this should be placed somewhere else, and use a
	// resolved.
        for s in config.find_keys(&DEPENDENCIES).unwrap_or(Vec::new()) {
            let a = [&TMP,s.as_str()];
            let k = Key::new(&a);
	    let d = config.get_string(&k)?;
	    let mut pb = PathBuf::new();
	    pb.push(whileyhome);
	    pb.push("repository");
	    pb.push(format!("{}-v{}.zip",&s,&d));
	    // FIXME: whileypath should be Vec of PathBuf
	    let arg = pb.into_os_string().into_string().unwrap();
	    whileypath.push(arg);
        }
	// Construct new instance on the heap
	let instance = Box::new(WhileyPlatform{name,linking,source,target,includes,whileypath});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
