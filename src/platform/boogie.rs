use std::error::Error;
use std::path::{Path,PathBuf};
use crate::config;
use crate::config::{Config,Key};
use crate::build;
use crate::build::{PACKAGE_NAME,Artifact};
use crate::platform;
use crate::platform::{PluginError,whiley};

pub static VERIFY_DEFAULT : bool = true;
pub static VERBOSE_DEFAULT : bool = false;
pub static DEBUG_DEFAULT : bool = false;
pub static TIMEOUT_DEFAULT : i64 = 10; // s
pub static ARRAYTHEORY_DEFAULT : bool = true;

static BUILD_BOOGIE_TARGET : Key = Key::new(&["build","boogie","target"]);
static BUILD_BOOGIE_VERIFY : Key = Key::new(&["build","boogie","verify"]);
static BUILD_BOOGIE_VERBOSE : Key = Key::new(&["build","boogie","verbose"]);
static BUILD_BOOGIE_DEBUG : Key = Key::new(&["build","boogie","debug"]);
static BUILD_BOOGIE_TIMEOUT : Key = Key::new(&["build","boogie","timeout"]);
static BUILD_BOOGIE_ARRAYTHEORY : Key = Key::new(&["build","boogie","useArrayTheory"]);
static BUILD_BOOGIE_PROVERLOG : Key = Key::new(&["build","boogie","proverLog"]);
static BUILD_BOOGIE_PROVERNAME : Key = Key::new(&["build","boogie","proverName"]);

// ========================================================================
// Platform
// ========================================================================

/// Identify the necessary dependencies (from Maven central) necessary
/// to run the WhileyCompiler.
static MAVEN_DEPS : &'static [&str] = &[
    whiley::MAVEN_DEPS[0], // jmodelgen
    whiley::MAVEN_DEPS[1], // wyc
    "org.whiley:wyboogie:0.4.1",
];

pub struct BoogiePlatform {
    name: String,
    source: String,
    binary: String,
    target: String,
    verify: bool,
    verbose: bool,
    debug: bool,
    timeout: i64,
    array_theory: bool,
    prover_log: Option<String>,
    prover_name: Option<String>
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
	args.push(format!("--output={}",self.name));
	args.push(format!("--wyildir={}",self.binary));
	args.push(format!("--bpldir={}",self.target));
	args.push(format!("--timeout={}",self.timeout));
	// Verify
	if !self.verify {
	    // Enable linking
	    args.push("--noverify".to_string());
	}
	// Verbose
	if self.verbose {
	    args.push("--verbose".to_string());
	}
	// Debug
	if self.debug {
	    args.push("--debug".to_string());
	}
	// useArrayTheory
	if self.array_theory {
	    args.push("--useArrayTheory".to_string());
	}
	// Prover log
	if self.prover_log.is_some() {
	    args.push(format!("--proverLog={}",self.prover_log.as_ref().unwrap().to_string()));
	}
	// Prover name
	if self.prover_name.is_some() {
	    args.push(format!("--proverName={}",self.prover_name.as_ref().unwrap().to_string()));
	}
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
	//
	artifacts
    }
    fn process(&self, output: &str) -> Result<Vec<build::Marker>,Box<dyn Error>> {
	/// FIXME: this is broken!
	let path = PathBuf::from(self.source);
	match whiley::parse_output(&path,output) {
	    Some(markers) => Ok(markers),
	    None => {
	        Err(Box::new(PluginError{name:"wyboogie".to_string(),message: output.to_string()}))
	    }
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
	let source = config.get_string(&whiley::BUILD_WHILEY_SOURCE).unwrap_or(whiley::SOURCE_DEFAULT.to_string());
	let binary = config.get_string(&whiley::BUILD_WHILEY_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	let target = config.get_string(&BUILD_BOOGIE_TARGET).unwrap_or(whiley::TARGET_DEFAULT.to_string());
	let verify = config.get_bool(&BUILD_BOOGIE_VERIFY).unwrap_or(VERIFY_DEFAULT);
	let verbose = config.get_bool(&BUILD_BOOGIE_VERBOSE).unwrap_or(VERBOSE_DEFAULT);
	let debug = config.get_bool(&BUILD_BOOGIE_DEBUG).unwrap_or(DEBUG_DEFAULT);
	let timeout = config.get_int(&BUILD_BOOGIE_TIMEOUT).unwrap_or(TIMEOUT_DEFAULT);
	let array_theory = config.get_bool(&BUILD_BOOGIE_ARRAYTHEORY).unwrap_or(ARRAYTHEORY_DEFAULT);
	let prover_log = config.get_string(&BUILD_BOOGIE_PROVERLOG).ok();
	let prover_name = config.get_string(&BUILD_BOOGIE_PROVERNAME).ok();
	// Construct new instance on the heap
	let instance = Box::new(BoogiePlatform{name,source,binary,target,verify,verbose,debug,timeout,array_theory,prover_log,prover_name});
	// Return generic instance
	Ok(platform::Instance::Java(instance))
    }
}

pub const DESCRIPTOR : Descriptor = Descriptor{};
