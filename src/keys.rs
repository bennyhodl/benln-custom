use lightning::io::Write;
use rand::{thread_rng, RngCore};
use std::fs::{self, File};

// If we're restarting and already have a key seed, read it from disk. Else,
// create a new one.
pub fn ben_keys(path: &str) -> [u8; 32] {
    let keys_seed_path = format!("{}/keys_seed", path.clone());

    let keys_seed = if let Ok(seed) = fs::read(keys_seed_path.clone()) {
        println!("Using keys at: {}", keys_seed_path);
        assert_eq!(seed.len(), 32);
        let mut key = [0; 32];
        key.copy_from_slice(&seed);
        key
    } else {
        println!("Creating keys at: {}", keys_seed_path);
        let mut key = [0; 32];
        thread_rng().fill_bytes(&mut key);
        match File::create(keys_seed_path.clone()) {
            Ok(mut f) => {
                f.write_all(&key)
                    .expect("Failed to write node keys seed to disk");
                f.sync_all().expect("Failed to sync node keys seed to disk");
            }
            Err(e) => {
                println!(
                    "ERROR: Unable to create keys seed file {}: {}",
                    keys_seed_path, e
                );
                [0; 32];
            }
        }
        key
    };
    keys_seed
}
