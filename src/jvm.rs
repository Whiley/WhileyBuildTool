use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

pub struct Jvm<T: AsRef<Path>> {
    classpath: Vec<T>    
}

impl<T: AsRef<Path>> Jvm<T> {
    pub fn new(classpath: Vec<T>) -> Self {
	Jvm{classpath}
    }

    pub fn exec(self, _args: &[&str]) {
	let mut args = Vec::new();	
	// Configure classpath
	let mut cp = String::new();
	//
	for c in self.classpath {
	    if cp.len() > 0 {
		cp.push_str(classpath_sep());
	    }
	    cp.push_str(c.as_ref().to_str().unwrap());
	}
	//
	args.push("-cp");
	args.push(cp.as_str());
	// Configure launcher
	args.push("wycli.Main");
	args.extend_from_slice(_args);
	// Run Java.
	let output = Command::new("java").args(args).output().expect("Java is not installed");
	//
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();	
    }
}

#[cfg(not(target_os = "windows"))]
pub fn classpath_sep() -> &'static str {
    ":"
}

#[cfg(target_os = "windows")]
pub fn classpath_sep() -> &'static str {
    ";"
}

