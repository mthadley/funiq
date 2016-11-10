use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
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
        let hash = File::open(&path).and_then(hash_file)
            .map_err(|e| {
                Error {
                    inner: e,
                    path: Some(path.to_owned()),
                }
            })?;

        if unique.contains_key(&hash) {
            duplicate.push(path);
        } else {
            unique.insert(hash, path);
        }
    }

    Ok((unique.values().cloned().collect(), duplicate))
}

fn hash_file(file: File) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    for b in file.bytes() {
        b?.hash(&mut hasher);
    }
    Ok(hasher.finish())
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.path {
            Some(ref path) => write!(f, "{}: {}", path, self.inner),
            _ => self.inner.fmt(f),
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
