use std::error;
use std::fmt;
use std::path::{PathBuf};
use toml;
use toml::{Value};

// ===================================================================
// Errors
// ===================================================================

type ParseError = toml::de::Error;

pub enum Type {
    Bool,
    Int,
    String,
    StringArray
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => {
                write!(f, "bool")
            }
	    Type::Int => {
                write!(f, "int")
            }
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
    Invalid(String),
    Expected(Type,String),
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
pub struct Key<'a>(&'a [&'a str]);

impl<'a> Key<'a> {
    pub const fn new(path: &'a [&'a str]) -> Self {
	Key(path)
    }
    pub fn to_vec(&self) -> Vec<&'a str> {
    	self.0.to_vec()
    }
}

impl<'a> fmt::Display for Key<'a> {
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

    /// Responsible for extracting a boolean associated with a given key.
    pub fn get_bool(&self, key: &Key) -> Result<bool,Error> {
	let val = match self.get_key(key) {
            None => {
		return Err(Error::Invalid(key.to_string()));
            }
            Some(v) => v.as_bool()
	};
	match val {
            Some(v) => Ok(v),
            None => Err(Error::Expected(Type::Bool,key.to_string()))
	}
    }    

    /// Responsible for extracting an integer associated with a given key.
    pub fn get_int(&self, key: &Key) -> Result<i64,Error> {
	let val = match self.get_key(key) {
            None => {
		return Err(Error::Invalid(key.to_string()));
            }
            Some(v) => v.as_integer()
	};
	match val {
            Some(v) => Ok(v),
            None => Err(Error::Expected(Type::Int,key.to_string()))
	}
    }
    
    /// Responsible for extracting a string associated with a given key.
    pub fn get_string(&self, key: &Key) -> Result<String,Error> {
	let val = match self.get_key(key) {
            None => {
		return Err(Error::Invalid(key.to_string()));
            }
            Some(v) => v.as_str()
	};
	match val {
            Some(v) => Ok(v.to_string()),
            None => Err(Error::Expected(Type::String,key.to_string()))
	}
    }
    
    /// Responsible for extracting a string array associated with a given key.
    pub fn get_string_array(&self, key: &Key) -> Result<Vec<String>,Error> {
	// Sanity check key exists
	let val = match self.get_key(key) {
            None => {
		return Err(Error::Invalid(key.to_string()));
            }
            Some(v) => v.as_array()
	};
	// Sanity check value is array
	let arr : &Vec<Value> = match val {
            None => {
		return Err(Error::Expected(Type::StringArray,key.to_string()));
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
                    return Err(Error::Expected(Type::StringArray,key.to_string()));
		}
		Some(v) => v                
            };
            res.push(s.to_string());
	}
	//
	Ok(res)
    }

    /// Responsible for getting the values of all subkeys within a given key.
    pub fn get_strings(&self, key: &Key) -> Result<Vec<(String,String)>,Error> {
	// Determine matching subkeys
	let subkeys = self.find_keys(key)?;
	// Convert key into a vector
	let mut subkey = key.to_vec();
	// Vec to hold results
	let mut pairs = Vec::new();	
	// Iterate each key extracting its value as a string.
	for i in 0..subkeys.len() {
	    let s = subkeys.get(i).unwrap();
	    subkey.push(s);
	    let sk = Key::new(&subkey);
	    let val = self.get_string(&sk)?;
	    pairs.push((s.to_string(),val));
	    subkey.pop();
	}
	// Done
	Ok(pairs)
    }

    /// Responsible for get the value of a given key as a path.
    pub fn get_path(&self, key: &Key) -> Result<PathBuf,Error> {
	let value = self.get_string(key)?;
	let mut result = PathBuf::new();
	// Split based on '/'
	for s in value.split("/") {
	    result.push(s);
	}
	// done
	Ok(result)
    }
    
    /// Responsible for identifying keys contained (directly) within
    /// this key.
    pub fn find_keys(&self, key: &Key) -> Result<Vec<String>,Error> {
        // Sanity check key exists
	let val = match self.get_key(key) {
            None => {
		return Err(Error::Invalid(key.to_string()));
            }
            Some(v) => v.as_table().ok_or(Error::Invalid(key.to_string()))?
	};
        // Extract keys!
        let mut keys = Vec::new();
        for (k,_) in val {
            keys.push(k.clone());
        }
        // Done
        Ok(keys)
    }
    
    /// Responsible for traversing the TOML tree and extracting the
    /// desired value (if it exists).    
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
