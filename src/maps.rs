use crate::data;
use rand::seq::SliceRandom;

pub fn get_map_list() -> Vec<String> {
    let mut rdr = csv::Reader::from_reader(data::MAPS.as_bytes());
    let mut maps = Vec::new();

    for record in rdr.records() {
        let result = record.unwrap();
        maps.push(result.as_slice().to_string());
    }
    shuffle_maps(&mut maps);

    return maps;
}

pub fn shuffle_maps(maps: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    maps.shuffle(&mut rng);
}
