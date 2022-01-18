use std::error;
use std::fmt;
use toml;
use toml::{Value};
use crate::platform::{Platform,PlatformRegistry};

// ===================================================================
// Errors
// ===================================================================

type ParseError = toml::de::Error;

pub enum Type {
    String,
    StringArray
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::String => {
                write!(f, "string")
            }
            Type::StringArray => {
                write!(f, "string array")
            }
        }
    }
}

pub enum Error {
    ParseError(ParseError),
    Invalid(Key),
    Expected(Type,Key),
    UnknownPlatform(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error reading wy.toml file!")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(p) => {
                write!(f,"{}",p)
            }
            Error::Invalid(k) => {
                write!(f,"invalid key \"{}\"",k)
            }
            Error::Expected(t,k) => {
                write!(f,"expected {} for \"{}\"",t,k)
            }
            Error::UnknownPlatform(s) => {
                write!(f,"unknown build platform \"{}\"",s)
            }
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::ParseError(err)
    }
}

impl error::Error for Error {}

// ===================================================================
// Keys
// ===================================================================

#[derive(Clone,Copy,Debug)]
pub struct Key(&'static [&'static str]);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0[0])?;        
        for s in &self.0[1..] {
            write!(f,".{}",s)?;            
        }
        Ok(())
    }
}

// ===================================================================
// Config
// ===================================================================

static PACKAGE_NAME : Key = Key(&["package","name"]);
static PACKAGE_AUTHORS : Key = Key(&["package","authors"]);
static PACKAGE_VERSION : Key = Key(&["package","version"]);
static BUILD_PLATFORMS : Key = Key(&["build","platforms"]);

pub struct Package {    
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
}

pub struct Build<'a> {
    pub platforms: Vec<&'a dyn Platform>
}

pub struct Config<'a> {
    pub package: Package,
    pub build: Build<'a>
}

impl<'a> Config<'a> {
    /// Parse a give string into a build configuration.
    pub fn from_str(contents: &str, registry: &'a PlatformRegistry<'a>) -> Result<Config<'a>,Error> {
        // Parse TOML configuration file
	let toml: Value = toml::from_str(contents)?;
        // Extract all required keys
        let name = get_string(&toml,&PACKAGE_NAME)?;
        let authors = get_string_array(&toml,&PACKAGE_AUTHORS)?;
        let version = get_string(&toml,&PACKAGE_VERSION)?;
        let platforms = get_string_array(&toml,&BUILD_PLATFORMS)?;
	// Construct package information
	let package = Package{name, authors, version};
        // Construct build information
        for p in &platforms {
            let platform = match registry.get(p) {
                None => {
                    return Err(Error::UnknownPlatform(p.to_string()));
                }
                Some(v) => v
            };
            println!("GOT: {}",platform.name());
        }
	let build = Build{platforms:Vec::new()};
	// Sanity check configuration!
	// Done
	return Ok(Config{package,build});
    }
}

// ===================================================================
// Generic Helpers
// ===================================================================

fn get_key<'a>(toml: &'a Value, key: &Key) -> Option<&'a Value> {
    let n = key.0.len();
    // Sanity check
    match n {
        0 => None,
        _ => {
            // Extract key
            let mut val = toml;
            // Traverse key
            for i in 0..n {                
                val = match val.get(key.0[i]) {
                    None => {
                        return None;
                    }
                    Some(v) => v
                };
            }
            //
            Some(val)    
        }
    }
}
  
    
fn get_string<'a>(toml: &'a Value, key: &Key) -> Result<String,Error> {
    let val = match get_key(toml,key) {
        None => {
            return Err(Error::Invalid(*key));
        }
        Some(v) => v.as_str()
    };
    match val {
        Some(v) => Ok(v.to_string()),
        None => Err(Error::Expected(Type::String,*key))
    }
}

fn get_string_array<'a>(toml: &'a Value, key: &Key) -> Result<Vec<String>,Error> {
    // Sanity check key exists
    let val = match get_key(toml,key) {
        None => {
            return Err(Error::Invalid(*key));
        }
        Some(v) => v.as_array()
    };
    // Sanity check value is array
    let arr : &Vec<Value> = match val {
        None => {
            return Err(Error::Expected(Type::StringArray,*key));
        }        
        Some(v) => {
            v
        }                
    };
    // Sanity check value is string array    
    let mut res : Vec<String> = Vec::new();
    //
    for v in arr {
        let s = match v.as_str() {
            None => {
                return Err(Error::Expected(Type::StringArray,*key));
            }
            Some(v) => v                
        };
        res.push(s.to_string());
    }
    //
    Ok(res)
}
