use clap::{App, AppSettings};
use j4rs::{Instance, InvocationArg, Jvm, JvmBuilder, errors::J4RsError};
use j4rs::MavenArtifact;

fn main() -> Result<(),J4RsError> {
    // Create a JVM
    let jvm = JvmBuilder::new().build()?;
    //
    let empty_array = jvm.create_java_array("java.lang.String", &Vec::new())?;
    let http_components = MavenArtifact::from("org.apache.httpcomponents:httpclient:4.5.13");
    let jbfs_artifact =  MavenArtifact::from("org.whiley:jbuildfs:1.0.1");
    let wycc_artifact = MavenArtifact::from("org.whiley:wycc:0.9.9");
    let wycli_artifact = MavenArtifact::from("org.whiley:wycli:0.9.9");
    jvm.deploy_artifact(&http_components)?;        
    jvm.deploy_artifact(&jbfs_artifact)?;    
    jvm.deploy_artifact(&wycc_artifact)?;
    jvm.deploy_artifact(&wycli_artifact)?;    
    //
    let _static_invocation_result = jvm.invoke_static(
	"wycli.Main",
	"main",
	&[InvocationArg::from(empty_array)],
    )?;
    //
    println!("GOT HERE");
    //
    Ok(())
}


// pub fn main() {
//     // Parse command-line arguments
//     let matches = App::new("wy")
// 	.about("Whiley's Build Tool and Package Manager")
// 	.version("0.6.0")
// 	.setting(AppSettings::SubcommandRequiredElseHelp)
// 	.subcommand(
// 	    App::new("build").about("Build local package(s)"))
// 	.subcommand(
// 	    App::new("clean").about("Remove all generated build artifact(s)"))	
// 	.get_matches();
//     // Dispatch on outcome
//     match matches.subcommand() {
// 	("build", Some(_)) => {
// 	    println!("Build not implemented yet!");
// 	}
// 	("clean", Some(_)) => {
// 	    println!("Clean not implemented yet!");
// 	}
// 	_ => unreachable!()
//     }
//     //
//}
