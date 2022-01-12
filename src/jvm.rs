use std::process::Command;
use std::io::{self, Write};

pub struct Jvm<'a> {
    classpath: Vec<&'a str>    
}

impl<'a> Jvm<'a> {
    pub fn new(classpath: Vec<&'a str>) -> Self {
	Jvm{classpath}
    }

    pub fn exec(self, args: &[&str]) {
	// Run Java.
	let output = Command::new("java").args(args).output().expect("Java is not installed");
	//
	io::stdout().write_all(&output.stdout).unwrap();
    }
}


