extern crate rayon;

use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

pub struct Error {
    inner: io::Error,
    path: Option<String>,
}

/// Process files in parallel.
pub fn process_files(paths: Vec<String>) -> Result<(Vec<String>, Vec<String>), Error> {
    let mut seen: HashSet<u64> = HashSet::new();

    let (duplicate, unique) = paths
        .par_iter()
        .map(|path| {
            File::open(&path).and_then(hash_file).map_err(|e| Error {
                inner: e,
                path: Some(path.clone()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .zip(paths)
        .partition(|&(hash, _)| !seen.insert(hash));

    Ok((map_snd(unique), map_snd(duplicate)))
}

fn hash_file(file: File) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    for b in file.bytes() {
        b?.hash(&mut hasher);
    }
    Ok(hasher.finish())
}

fn map_snd<T, U>(pairs: Vec<(T, U)>) -> Vec<U> {
    pairs.into_iter().map(|(_, s)| s).collect()
}

impl fmt::Debug for Error {
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
