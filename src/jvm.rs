use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub struct Jvm<T: AsRef<Path>, K: AsRef<OsStr>, V: AsRef<OsStr>> {
    classpath: Vec<T>,
    env: Vec<(K,V)>
}

impl<T: AsRef<Path>, K: AsRef<OsStr>, V: AsRef<OsStr>> Jvm<T,K,V> {
    pub fn new(classpath: Vec<T>, env: Vec<(K,V)>) -> Self {
	Jvm{classpath,env}
    }

    pub fn exec(self, _args: &[&str]) -> String {
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
	args.extend_from_slice(_args);
	// Run Java!
	let output = Command::new("java")
	    .args(args)
	    .envs(self.env)
	    .output()
	    .expect("Java is not installed");
	// Merge stdout/stderr together
	let mut r = String::from_utf8(output.stdout).unwrap();
	r.push_str(&String::from_utf8(output.stderr).unwrap());
	// Done
	r
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
