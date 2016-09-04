use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::{self, Read};

#[derive(Debug)]
pub struct Error {
    inner: io::Error,
    path: Option<String>,
}

#[inline]
pub fn process_files<'a>(paths: &'a [String]) -> Result<(Vec<&'a str>, Vec<&'a str>), Error> {
    let mut unique: HashMap<u64, &str> = HashMap::new();
    let mut duplicate: Vec<&str> = Vec::new();

    for path in paths {
        match File::open(&path).and_then(hash_file) {
            Ok(hash) => {
                if unique.contains_key(&hash) {
                    duplicate.push(path);
                } else {
                    unique.insert(hash, path);
                }
            }
            Err(e) => {
                return Err(Error {
                    inner: e,
                    path: Some(path.to_owned()),
                })
            }
        }
    }

    Ok((unique.values().map(|s| *s).collect(), duplicate))
}

fn hash_file(file: File) -> io::Result<u64> {
    let mut s = SipHasher::new();
    for b in file.bytes() {
        try!(b).hash(&mut s);
    }
    Ok(s.finish())
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref path) = self.path {
            write!(f, "{}: {}", path, self.inner)
        } else {
            write!(f, "{}", self.inner)
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            inner: e,
            path: None,
        }
    }
}
