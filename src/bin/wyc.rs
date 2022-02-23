//use clap::{App, AppSettings};
use std::env;
use log::LevelFilter;
use whiley::platform::whiley::MAVEN_DEPS;
use whiley::jvm::Jvm;
use whiley::{init_logging,init_whileyhome,init_classpath};

fn main() {
    // Initialise logging
    init_logging(LevelFilter::Info);
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Initialise classpath as necessary.  This will download Jar
    // files from Maven central (if not already cached).    
    let cp = init_classpath(&whileyhome,MAVEN_DEPS);
    // Construct JVM runner
    let jvm = Jvm::new(cp,vec![("WHILEYHOME",&whileyhome)]);
    // Extract command-line arguments
    let mut args : Vec<String> = env::args().collect();
    // Strip first element (is this program)
    args[0]="wyc.Main".to_string();
    // Convert into Vec<&str> for exec
    let str_args : Vec<&str> = args.iter().map(String::as_str).collect();
    // Go!
    jvm.exec(&str_args);
}
