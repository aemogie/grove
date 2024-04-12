use std::path::PathBuf;

use digest::Digest;

#[derive(Debug)]
pub struct Store<D: Digest> {
    path: PathBuf,
    algo: D,
}

impl<D: Digest> Store<D> {
    pub fn new(path: PathBuf, algo: D) -> Self {
        Self { path, algo }
    }
}
