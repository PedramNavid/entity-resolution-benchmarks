use entity_rust::*;
use log::info;
fn main() {
    env_logger::init();

    info!("Reading files");

    let mut dblp = read_csv("../data/DBLP2utf8.csv").unwrap();
    let acm = read_csv("../data/ACM.csv").unwrap();
    dblp.extend(acm);
    info!("Finished reading files");

    let records = dblp.values().collect::<Vec<&Record>>();

    let rev_idx = reverse_index(records, 10);
    info!("Found {} tokens", rev_idx.len());

    let scores = score_blocks(&rev_idx, &dblp);
    info!("Found {} scores", scores.len());
}
