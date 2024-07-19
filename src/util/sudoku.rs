use std::collections::{HashMap, HashSet};
use crate::util::units::get_squares;

pub fn initialize_sudoku(board: &str) -> HashMap<String, HashSet<char>> {
    let mut sudoku = HashMap::new();
    let digits: HashSet<char> = "123456789".chars().collect();
    let squares = get_squares();

    for (i, ch) in board.chars().enumerate() {
        let key = &squares[i];
        if ch == '.' {
            sudoku.insert(key.clone(), digits.clone());
        } else {
            let mut set = HashSet::new();
            set.insert(ch);
            sudoku.insert(key.clone(), set);
        }
    }
    sudoku
}

pub fn not_consistent(
    sudoku: &mut HashMap<String, HashSet<char>>,
    _units: &HashMap<String, Vec<Vec<String>>>,
    peers: &HashMap<String, HashSet<String>>,
) -> bool {
    let mut changes = true;
    while changes {
        changes = false;
        for (s, value_set) in sudoku.clone().iter() {
            if value_set.len() == 1 {
                let value = *value_set.iter().next().unwrap();
                for peer in &peers[s] {
                    if sudoku[peer].contains(&value) {
                        sudoku.get_mut(peer).unwrap().remove(&value);
                        if sudoku[peer].is_empty() {
                            return true;
                        }
                        changes = true;
                    }
                }
            }
        }
    }
    false
}

pub fn sudoku_to_string(sudoku: &HashMap<String, HashSet<char>>) -> String {
    let squares = get_squares();
    squares.iter().map(|s| {
        let value = sudoku[s].iter().next().unwrap();
        *value
    }).collect()
}
