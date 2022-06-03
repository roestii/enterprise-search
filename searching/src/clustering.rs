use std::{path::PathBuf, fs::{self, File}, io::{Read, Write}, collections::{HashMap, HashSet}, cell::RefCell, borrow::BorrowMut};

type Cluster = Vec<Element>;
type TermFrequencies = (HashSet<String>, Vec<Vec<RefCell<i32>>>);

trait Clusterer {
    fn measure_performance(&self) -> i32;
    fn cluster(&mut self) -> &Vec<Cluster>;
}

enum Entry {
    Term(i32),
    Empty(i32),
}

#[derive(Debug, Clone)]
pub struct Element(PathBuf);

pub struct TermWeighter {
    elements: Vec<Element>,
    unit_length: i32,
    term_freqs: TermFrequencies,
}

pub fn calculate_idf(term: String, doc_count: i32, terms_in_documents: &HashMap<String, i32>) -> f64 {
    let term_count = terms_in_documents.get(&term).unwrap();
    return ((doc_count as f64) / (*term_count as f64)).log10();
}

pub fn calculate_tf_idf(term: String, term_count: i32, term_idf_map: &HashMap<String, f64>) -> f64 {
    let tf = (term_count as f64) / (term_idf_map.iter().count() as f64);
    return tf * term_idf_map.get(&term).unwrap();
}

impl TermWeighter {
    pub fn new(elements: Vec<Element>, unit_length: i32) -> Self {
        Self {
            elements,
            unit_length,
            term_freqs: (HashSet::new(), Vec::new()),
        }
    }

    //Returns the term and the corresponding amount of documents it occurs in
    pub fn get_terms(&self) -> HashMap<String, i32> {
        let term_counts: Vec<(PathBuf, Vec<String>)> = self.elements
            .iter()
            .filter_map(|element| element.clone().try_into().ok())
            .take(200)
            .collect();

        let terms: HashSet<String> = HashSet::from_iter(term_counts.iter().map(|entry| entry.1.clone()).flatten());

        let terms_map: HashMap<String, i32> = terms.iter()
            .fold(HashMap::new(), |mut acc, term| {
                let document_count = term_counts.iter()
                    .filter(|(_, term_vector)| {
                        term_vector.iter()
                            .filter(|t| *t == term)
                            .count() > 0
                    })
                    .count();
                acc.insert(term.to_string(), document_count as i32);
                acc
            });

        terms_map
    }

    pub fn calculate_term_freqs(mut self) {
        let not_weighted: Vec<HashMap<String, RefCell<i32>>> = self.elements
            .iter()
            .filter_map(|element| element.clone().try_into().ok())
            .take(200)
            .collect();

        let terms = self.get_terms();
        println!("Terms generated");
        let mut term_file = File::create("/home/louis/Dev/enterprise-search/master/vectors/terms.json")
            .expect("Failed to create term file");
        let term_content = serde_json::to_string(&terms)
            .expect("Failed to parse vec to JSON string");
        term_file.write(term_content.as_bytes())
            .expect("Failed to write to term file");

        let mut initial_map = HashMap::new();

        for term in &terms {
            initial_map.insert(term.0, 0 as f64);
        }

        let mut term_idf_map = HashMap::new();
        let doc_count = self.elements.iter().count();

        for term in &terms {
            let idf = calculate_idf(term.0.to_string(), doc_count as i32, &terms);
            term_idf_map.insert(term.0.to_string(), idf);
        }

        let mut term_vectors: Vec<Vec<f64>> = Vec::new();
        for (i, term_vector) in not_weighted.iter().enumerate() {
            if i % 100 == 0 && i != 0 {
                let mut term_vector_file = File::create(format!("/home/louis/Dev/enterprise-search/master/vectors/file_vector_{}-{}.json", i - 100, i))
                    .expect("Failed to create file");
                let lines = serde_json::to_string(&term_vectors).unwrap();

                term_vector_file.write(lines.as_bytes()).expect("Cannot write to file");
                term_vectors = Vec::new();
            }

            let term_idfs = term_vector.iter()
                .fold(initial_map.clone(), |mut acc, (key, value)| {
                    let thing = acc.get_mut(key).unwrap();
                    let tf_idf = calculate_tf_idf(key.to_string(), value.clone().take(), &term_idf_map);
                    *thing = tf_idf;
                    acc
                })
                .values()
                .map(|value| value.clone())
                .collect();
            println!("Hello");


            term_vectors.push(term_idfs);
        }
        let mut term_vector_file = File::create(format!("/home/louis/Dev/enterprise-search/master/vectors/file_vector_last.json"))
            .expect("Failed to create file");
        let lines = serde_json::to_string(&term_vectors).unwrap();
        term_vector_file.write(lines.as_bytes()).expect("Failed writing to file");
    }
}



//To terms
impl TryFrom<Element> for (PathBuf, Vec<String>) {
    type Error = Box<dyn std::error::Error>;

    fn try_from(Element(path_buf): Element) -> Result<Self, Self::Error> {
        let mut file = File::open(path_buf.as_path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let content = content.split(" ")
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        Ok((path_buf, content))
    }
}

impl TryFrom<Element> for HashMap<String, RefCell<i32>> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(Element(path_buf): Element) -> Result<Self, Self::Error> {
        let mut file = File::open(path_buf.as_path())?;
        let mut content = String::new(); 
        file.read_to_string(&mut content)?;

        let content = content.split(" ")
            .map(|word| word.to_string())
            .collect::<Vec<String>>(); 

        let term_freq: HashMap<String, RefCell<i32>> = content.iter()
            .fold(HashMap::new(), |mut freqs, word| {
                match freqs.get(word) {
                    Some(value) => {
                        *value.borrow_mut() += 1;
                    },
                    None => { freqs.insert(word.to_string(), RefCell::new(1)); },
                }

                freqs
            });

        Ok(term_freq)
    }
}

impl Default for TermWeighter {
    fn default() -> Self {
        let article_paths = fs::read_dir("/home/louis/Dev/enterprise-search/master/preprocessed_data/searching")
            .expect("Default dir not available")
            .filter_map(|entry| entry.ok())
            .map(|entry| Element(entry.path()))
            .collect();

        Self {
            elements: article_paths,
            unit_length: 10,
            term_freqs: (HashSet::new(), Vec::new()),
        }
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
