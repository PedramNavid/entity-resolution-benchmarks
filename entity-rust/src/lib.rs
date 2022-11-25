use regex::Regex;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
};

use log::debug;

use edit_distance::edit_distance;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    id: String,
    title: String,
    authors: String,
    venue: String,
}

pub type RecordSet = HashMap<String, Record>;
type ReverseIndex = HashMap<String, Vec<String>>;

pub fn read_csv(path: &str) -> Result<RecordSet, Box<dyn Error>> {
    let mut map = HashMap::new();

    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    for record in reader.deserialize() {
        let record: Record = record?;
        map.insert(record.id.clone(), record);
    }
    Ok(map)
}

fn clean_split_text(text: &str) -> Vec<String> {
    let re = Regex::new(r"[^\w,]").unwrap();
    let trimmed = re.replace_all(text, "").to_lowercase();

    trimmed
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect()
}

///Tokenize a string into n-grams of a given size.
fn tokenize(text: &str, n: usize) -> HashSet<String> {
    let mut tokens = HashSet::new();
    let clean_text = clean_split_text(text);
    for name in clean_text {
        let mut chars = name.chars();
        let mut ngram = String::new();
        for _ in 0..n {
            let next_char = chars.next();
            match next_char {
                Some(c) => {
                    ngram.push(c);
                }
                None => {}
            };
        }
        tokens.insert(ngram.clone());
        for c in chars {
            ngram.remove(0);
            ngram.push(c);
            tokens.insert(ngram.clone());
        }
    }
    tokens
}

pub fn reverse_index(records: Vec<&Record>, n: usize) -> ReverseIndex {
    let rev_idx = records
        .iter()
        .flat_map(|r| {
            let tokens = tokenize(&r.authors, n);
            tokens
                .iter()
                .map(|t| (t.clone(), r.id.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .fold(HashMap::new(), |mut acc, (token, id)| {
            acc.entry(token).or_insert(Vec::new()).push(id);
            acc
        });
    rev_idx
}

fn score_lev_distance(a: &str, b: &str) -> f64 {
    1.0 - edit_distance(a, b) as f64 / (max(a.len(), b.len()) as f64)
}

pub fn score_blocks(rev_idx: &ReverseIndex, rec_set: &RecordSet) -> Vec<(String, String, f64)> {
    let mut scores = Vec::new();
    for (token, ids) in rev_idx {
        debug!("Token: {:?} with {:?} ids", token, ids.len());
        for (i, id1) in ids.iter().enumerate() {
            for id2 in ids.iter().skip(i + 1) {
                let rec1 = rec_set.get(id1).unwrap();
                let rec2 = rec_set.get(id2).unwrap();
                let score = score_lev_distance(&rec1.title, &rec2.title);
                scores.push((id1.clone(), id2.clone(), score));
            }
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let path = "../data/DBLP2utf8.csv";
        let reader = read_csv(path).unwrap();
        let path = "../data/Scholar.csv";
        let reader = read_csv(path).unwrap();
    }

    #[test]
    fn test_tokenize() {
        let input = "Ab'cdbcd, E[$fg";
        let tokens = tokenize(input, 3);
        assert_eq!(
            tokens,
            ["abc", "bcd", "cdb", "dbc", "efg"].map(String::from).into()
        )
    }

    #[test]
    fn test_edit() {
        assert_eq!(score_lev_distance("abc", "abc"), 1.0);
        assert_eq!(score_lev_distance("pedram navid", "pedram novar"), 0.75);
    }

    #[test]
    fn test_rev_idx() {
        let r1 = Record {
            id: "1".to_string(),
            title: "title".to_string(),
            authors: "john, doe".to_string(),
            venue: "venue".to_string(),
        };
        let r2 = Record {
            id: "2".to_string(),
            title: "title".to_string(),
            authors: "doe".to_string(),
            venue: "venue".to_string(),
        };

        let records = vec![&r1, &r2];
        let rev_idx = reverse_index(records, 3);
        assert_eq!(rev_idx["doe"], vec!["1", "2"]);
        assert_eq!(rev_idx["joh"], vec!["1"]);
        assert_eq!(rev_idx["ohn"], vec!["1"]);
    }

    #[test]
    fn test_score_blocks() {
        let path = "../data/DBLP21000.csv";
        let reader = read_csv(path).unwrap();
        let records: Vec<&Record> = reader.values().collect();
        let rev_idx = reverse_index(records, 10);

        let scores = score_blocks(&rev_idx, &reader);
        for score in scores {
            if score.2 >= 0.45 {
                debug!("{:?}", score);
            }
        }
    }
}
