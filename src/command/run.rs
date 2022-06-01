use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use log::info;
use crate::{init_classpath};
use crate::config::{Config,Key};
use crate::jvm;
use crate::jvm::{Jvm};
use crate::{init_registry};
use crate::build::{DEPENDENCIES, PACKAGE_NAME};
use crate::platform::whiley::{MAVEN_DEPS, BUILD_WHILEY_TARGET, BUILD_WHILEY_MAIN, MAIN_DEFAULT, TARGET_DEFAULT};

pub const TMP : &'static str = "dependencies";

// Run command
pub fn run(whileyhome: &Path) -> Result<bool,Box<dyn Error>> {
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;
    // Extract build information
    let name = config.get_string(&PACKAGE_NAME)?;
    let target = config.get_path(&BUILD_WHILEY_TARGET).unwrap_or(PathBuf::from(TARGET_DEFAULT));
    let main = config.get_string(&BUILD_WHILEY_MAIN).unwrap_or(MAIN_DEFAULT.to_string());
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
    // Initialise platform registry
    let registry = init_registry();
    // Initialise classpath as necessary.  This will download Jar
    // files from Maven central (if not already cached).
    let cp = init_classpath(&whileyhome,MAVEN_DEPS)?;
    // Construct JVM runner
    let jvm = Jvm::new(cp,vec![("WHILEYHOME",&whileyhome)]);
    //
    let mut args : Vec<&str> = Vec::new();
    // Class to invoke
    args.push("wyc.Executor");
    // Target name
    args.push("-o");
    args.push(&name);
    //
    let wyildir = format!("--wyildir={}",target.to_str().unwrap());
    args.push(&wyildir);
    // Whiley path
    let mut wypath = String::new();
    if whileypath.len() > 0 {
        wypath.push_str("--whileypath=");
        wypath.push_str(whileypath.get(0).unwrap());
        for e in &whileypath[1..] {
            wypath.push_str(jvm::classpath_sep());
            wypath.push_str(e);
        }
	args.push(&wypath);
    }
    // Target method
    args.push(&main);
    // Log Java command
    info!("Executing java {:?}",args);
    // Go!
    let output = jvm.exec(&args);
    //
    print!("{}",output);
    //
    Ok(true)
}
