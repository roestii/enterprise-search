use std::{fs::File, path::PathBuf};
use std::io::Read;

use crate::{Searcher, Config};

pub struct BasicSearcher;

impl Searcher for BasicSearcher {
    fn search(search_term: &str, config: Config) -> Vec<PathBuf> {
        let paths: Vec<PathBuf> = config.paths.into_iter()
            .flat_map(|path| {
                path.read_dir().expect("Failed to read from dir")
            })
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter(|path| {
                if path.path().to_str().contains(&search_term) {
                    return true;
                }

                let mut content = String::new();
                let mut file = File::open(path.path()).expect("Failed to open file");
                file.read_to_string(&mut content).expect("Failed to read from file");

                return content.contains(&search_term);
            })
            .map(|entry| entry.path())
            .collect();

        return paths;
    }
}

