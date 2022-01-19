use std::error;
use std::fmt;
use toml;
use toml::{Value};

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

impl Key {
    pub const fn new(path: &'static [&'static str]) -> Self {
	Key(path)
    }
}

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

/// Essentially a wrapper around a TOML value.
pub struct Config {
    toml: Value
}

impl Config {

    /// Parse a give string into a configuration.  Internally, this
    /// uses the TOML representation but clients of this module don't
    /// need to know this.
    pub fn from_str<'b>(contents: &'b str) -> Result<Config,Error> {
        // Parse TOML configuration file
	let toml: Value = toml::from_str(contents)?;
	// Done
	Ok(Config{toml})
    }
        
    pub fn get_string(&self, key: &Key) -> Result<String,Error> {
	let val = match self.get_key(key) {
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
    
    pub fn get_string_array(&self, key: &Key) -> Result<Vec<String>,Error> {
	// Sanity check key exists
	let val = match self.get_key(key) {
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
    
    fn get_key<'a>(&'a self, key: &Key) -> Option<&'a Value> {
	let n = key.0.len();
	// Sanity check
	match n {
            0 => None,
            _ => {
		// Extract key
		let mut val = &self.toml;
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
}
