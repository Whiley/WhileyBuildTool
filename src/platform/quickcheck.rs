use std::error::Error;
use std::path::{Path,PathBuf};
use glob::glob;
use crate::config;
use crate::config::{Config,Key};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::platform;
use crate::platform::{PluginError,whiley};
use crate::jvm;

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    whiley::MAVEN_DEPS[0], // jmodelgen
    whiley::MAVEN_DEPS[1], // wyc
];

pub struct QuickCheckPlatform {
    name: String,
    source: PathBuf,
    target: String,
    whileypath: Vec<String>
}

impl QuickCheckPlatform {
    fn match_includes(&self) -> Vec<String> {
	let mut matches = Vec::new();
        matches.push(self.name.clone());
        matches
    }
}

impl platform::JavaInstance for QuickCheckPlatform {
    fn name(&self) -> &'static str {
        "check"
    }
    fn dependencies(&self) -> &'static [&'static str] {
	MAVEN_DEPS
    }
    fn arguments(&self) -> Vec<String> {
        let mut args = Vec::new();
        // Class to invoke
        args.push("wyc.Check".to_string());
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
	// This platform generates no files
	Vec::new()
    }
    fn process(&self, output: &str) -> Result<Vec<build::Marker>,Box<dyn Error>> {
	match whiley::parse_output(&self.source,output) {
	    Some(markers) => Ok(markers),
	    None => {
		Err(Box::new(PluginError{name:"wyqc".to_string(),message: output.to_string()}))
	    }
	}
    }
}

// ========================================================================
// Initialiser
// ========================================================================

pub struct Descriptor {}

impl platform::Descriptor for Descriptor {
    fn apply<'a>(&self, config: &'a Config, whileyhome: &Path) -> Result<platform::Instance,config::Error> {
	// Extract configuration (if any)
        let name = config.get_string(&PACKAGE_NAME)?;
	let source = config.get_path(&whiley::BUILD_WHILEY_SOURCE).unwrap_or(PathBuf::from(whiley::SOURCE_DEFAULT));
	let target = config.get_string(&whiley::BUILD_WHILEY_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
        // Construct whileypath?
        let mut whileypath = Vec::new();
	// FIXME: this should be placed somewhere else, and use a
	// resolved.
        for s in config.find_keys(&whiley::DEPENDENCIES).unwrap_or(Vec::new()) {
            let a = [&whiley::TMP,s.as_str()];
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
	let instance = Box::new(QuickCheckPlatform{name,source,target,whileypath});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
