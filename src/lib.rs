extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures::future::join_all;
use futures_cpupool::CpuPool;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
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
pub fn process_files<'a>(paths: &'a [String]) -> Result<(Vec<&'a str>, Vec<&'a str>), Error> {
    let pool = CpuPool::new(get_num_cpu());

    let futures = paths.iter().cloned().map(|path| {
        pool.spawn_fn::<_, Result<_, ()>>(move || {
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

    let mut unique: HashMap<u64, &str> = HashMap::new();
    let mut duplicate: Vec<&str> = Vec::new();

    let mut results = join_all(futures).wait()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Unknown Error."))?;

    for (result, path) in results.drain(..).zip(paths) {
        let hash = result?;
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

fn get_num_cpu() -> usize {
    env::var("FUNIQ_NUM_CPU")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(4)
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
