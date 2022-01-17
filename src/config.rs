use toml::{Value};

// ===================================================================
// Config
// ===================================================================

pub struct Config {
    pub package: Package,
    pub build: Build
}

impl Config {
    /// Parse a give string into a build configuration.
    pub fn from_str(contents: &str) -> Config {
	let toml: Value = toml::from_str(contents).unwrap();
	// TODO: sanity check package information
	let package = Package::from_value(&toml["package"]);
	let build = Build::from_value(&toml["build"]);
	// Sanity check configuration!
	// Done
	return Config{package,build};
    }
}

// ===================================================================
// Package
// ===================================================================

pub struct Package {    
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
}

impl Package {
    pub fn from_value(toml: &Value) -> Package {
	let name = toml["name"].as_str().unwrap().to_string();
	let authors = parse_array_string(toml["authors"].as_array().unwrap());
	let version = toml["version"].as_str().unwrap().to_string();
	Package{name, authors, version}
    }
}

// ===================================================================
// Build
// ===================================================================

pub struct Build {
    pub platforms: Vec<Platform>
}

impl Build {
    pub fn from_value(toml: &Value) -> Build {
	return Build{platforms: Vec::new()};
    }
}

// ===================================================================
// Platform
// ===================================================================

pub struct Platform {
    pub name: String
}

// ===================================================================
// Generic Helpers
// ===================================================================

fn parse_array_string(arr: &Vec<Value>) -> Vec<String> {
    let mut res = Vec::new();
    //
    for v in arr {
	res.push(v.as_str().unwrap().to_string());
    }
    //
    res
}
