#![feature(option_result_contains)]

use std::path::PathBuf;
use std::env;


mod basic_file_search;
mod clustering;

use crate::basic_file_search::BasicSearcher;
use crate::clustering::ClustererFS;

#[derive(Debug)]
pub struct Config {
    search_results: u8,
    paths: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        let current_dir = env::current_dir().unwrap();

        Self {
            search_results: 10,
            paths: vec![current_dir],
        }
    }
}

pub trait Searcher {
    //gives back the path of the ten best search results
    fn search(search_term: &str, config: Config) -> Vec<PathBuf>;
}

fn search<S>(search_term: &str, config: Config) -> Vec<PathBuf> 
where
    S: Searcher
{
    S::search(search_term, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_file_searcher() {
        let config = Config::default();
        println!("{:?}", search::<BasicSearcher>("package", config));
    }

    #[test]
    fn test_default_clusterer() {
        let clusterer = ClustererFS::default(); 
        println!("{:?}", clusterer);
    }
}
