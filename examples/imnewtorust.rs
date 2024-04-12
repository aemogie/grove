use grimoire::Store;
use sha2::{Digest, Sha256};
use std::env;

fn main() {
    let store = Store::new(env::temp_dir().join("grimoire_dev"), Sha256::new());
    dbg!(&store);
}
