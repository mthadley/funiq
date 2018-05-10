extern crate futures;
extern crate futures_cpupool;
extern crate num_cpus;

use futures::Future;
use futures::future::join_all;
use futures_cpupool::CpuPool;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};
use std::env;

#[derive(Debug)]
pub struct Error {
    inner: io::Error,
    path: Option<String>,
}

/// Sets up a thread pool for parallel processing of files.
pub fn process_files(paths: Vec<String>) -> Result<(Vec<String>, Vec<String>), Error> {
    let pool = CpuPool::new(get_num_cpus());

    let futures = paths.clone().into_iter().map(|path| {
        pool.spawn_fn::<_, Result<_, ()>>(|| {
            let result = File::open(&path)
                .and_then(hash_file)
                .map_err(|e| {
                    Error {
                        inner: e,
                        path: Some(path),
                    }
                });

            Ok(result)
        })
    });

    let hashes = join_all(futures)
        .wait()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Unknown Error."))?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    let mut seen: HashSet<u64> = HashSet::new();
    let (duplicate, unique) = hashes
        .into_iter()
        .zip(paths)
        .partition(|&(hash, _)| {
            let is_duplicate = seen.contains(&hash);
            seen.insert(hash);
            is_duplicate
        });

    Ok((map_snd(unique), map_snd(duplicate)))
}

fn hash_file(file: File) -> io::Result<u64> {
    let mut hasher = DefaultHasher::new();
    for b in file.bytes() {
        b?.hash(&mut hasher);
    }
    Ok(hasher.finish())
}

fn get_num_cpus() -> usize {
    env::var("FUNIQ_NUM_CPU")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or_else(num_cpus::get)
}

fn map_snd<T, U>(pairs: Vec<(T, U)>) -> Vec<U> {
    pairs.into_iter().map(|(_, s)| s).collect()
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
