use std::{path::PathBuf, fs};

trait Clusterer {
    fn measure_performance(&self) -> i32;
    fn cluster(&mut self) -> &Vec<Cluster>;
}

#[derive(Debug)]
struct Element(PathBuf);

type Cluster = Vec<Element>;

impl From<Element> for Vec<String> {
    fn from(element: Element) -> Self {
        todo!()
    }
}

//Text clusterer with feature selection
#[derive(Debug)]
pub struct ClustererFS {
    file_paths: Vec<Element>,
    clusters: Vec<Cluster>,
}

impl Default for ClustererFS {
    fn default() -> Self {
        //TODO: Change this to searching dir
        let article_paths = fs::read_dir("/home/louis/Dev/enterprise-search/master/preprocessed_data/clustering")
            .expect("Default dir not available")
            .filter_map(|entry| entry.ok())
            .map(|entry| Element(entry.path()))
            .collect();

        Self {
            file_paths: article_paths,
            clusters: Vec::new(),
        }
    }
}

impl Clusterer for ClustererFS {
    fn measure_performance(&self) -> i32 {
        todo!()
    }

    fn cluster(&mut self) -> &Vec<Cluster> {
        todo!()
    }
}
