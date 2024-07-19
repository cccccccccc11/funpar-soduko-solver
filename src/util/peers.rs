use std::collections::{HashMap, HashSet};
use crate::util::units::get_units;

pub fn get_peers() -> HashMap<String, HashSet<String>> {
    let squares = crate::util::units::get_squares();
    let units = get_units();

    let mut peers = HashMap::new();
    for s in &squares {
        let mut peer_set = HashSet::new();
        for unit in &units[s] {
            for square in unit {
                if square != s {
                    peer_set.insert(square.clone());
                }
            }
        }
        peers.insert(s.clone(), peer_set);
    }
    peers
}
