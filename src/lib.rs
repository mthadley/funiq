use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::{self, Read};

#[derive(Debug)]
pub struct Error<'a> {
    inner: io::Error,
    path: &'a str,
}

#[inline]
pub fn process_files<'a>(paths: &[&'a str]) -> Result<(Vec<&'a str>, Vec<&'a str>), Error<'a>> {
    let mut unique = HashMap::new();
    let mut duplicate = Vec::new();

    for p in paths {
        let path = *p;
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
                    path: path,
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

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.inner)
    }
}
