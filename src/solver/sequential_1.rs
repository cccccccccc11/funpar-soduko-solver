use std::collections::{HashMap, HashSet};
use crate::util::{sudoku::*, peers::*, units::*};

pub fn solve_seq(board: &str) -> Vec<String> {
    let mut sudoku: HashMap<String, HashSet<char>> = initialize_sudoku(board);
    let units = get_units();
    let peers = get_peers();

    if not_consistent(&mut sudoku, &units, &peers) {
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
        return vec![sudoku_to_string(&sudoku)];
    }

    let (min_square, min_set) = sudoku
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .min_by_key(|(_, v)| v.len())
        .unwrap();

    let mut solutions = Vec::new();
    for &value in min_set {
        let mut sudoku_copy = sudoku.clone();
        sudoku_copy.get_mut(min_square).unwrap().clear();
        sudoku_copy.get_mut(min_square).unwrap().insert(value);

        if not_consistent(&mut sudoku_copy, units, peers) {
            continue;
        } else {
            solutions.extend(search(sudoku_copy, units, peers));
        }
    }
    solutions
}
