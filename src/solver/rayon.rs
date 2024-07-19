use std::collections::{HashMap, HashSet};
use rayon::prelude::*;


pub fn solve_par_rayon(board: &str) -> Vec<String> {
    let mut sudoku: HashMap<String, HashSet<char>> = crate::util::sudoku::initialize_sudoku(board);
    let units = crate::util::units::get_units();
    let peers = crate::util::peers::get_peers();

    if crate::util::sudoku::not_consistent(&mut sudoku, &units, &peers) {
        return vec![];
    }

    search(sudoku, &units, &peers)
}


fn search(
    sudoku: HashMap<String, HashSet<char>>,
    units: &HashMap<String, Vec<Vec<String>>>,
    peers: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    if sudoku.values().all(|v| v.len() == 1) {
        return vec![crate::util::sudoku::sudoku_to_string(&sudoku)];
    }

    let (min_square, min_set) = sudoku
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .min_by_key(|(_, v)| v.len())
        .unwrap();
    min_set
        .par_iter()
        .flat_map(|&value| {
            let mut sudoku_copy = sudoku.clone();
            sudoku_copy.get_mut(min_square).unwrap().clear();
            sudoku_copy.get_mut(min_square).unwrap().insert(value);

            if crate::util::sudoku::not_consistent(&mut sudoku_copy, units, peers) {
                vec![]
            } else {
                search(sudoku_copy, units, peers)
            }
        })
        .collect()
}
