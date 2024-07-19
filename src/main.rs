mod solver {
    pub mod crossbeam;
    pub mod rayon;
    pub mod sequential_1;
}

mod util;
mod sequential_solver;

use solver::crossbeam::solve_par_crossbeam;
use solver::rayon::solve_par_rayon;
use util::board::print_board;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use crate::util::board::format_board;

fn main() {
    println!(r#"
███████╗██╗   ██╗██████╗  ██████╗ ██╗  ██╗██╗   ██╗    ███████╗ ██████╗ ██╗    ██╗   ██╗███████╗██████╗
██╔════╝██║   ██║██╔══██╗██╔═══██╗██║ ██╔╝██║   ██║    ██╔════╝██╔═══██╗██║    ██║   ██║██╔════╝██╔══██╗
███████╗██║   ██║██║  ██║██║   ██║█████╔╝ ██║   ██║    ███████╗██║   ██║██║    ██║   ██║█████╗  ██████╔╝
╚════██║██║   ██║██║  ██║██║   ██║██╔═██╗ ██║   ██║    ╚════██║██║   ██║██║    ╚██╗ ██╔╝██╔══╝  ██╔══██╗
███████║╚██████╔╝██████╔╝╚██████╔╝██║  ██╗╚██████╔╝    ███████║╚██████╔╝███████╗╚████╔╝ ███████╗██║  ██║
╚══════╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝     ╚══════╝╚═════╝ ╚══════╝ ╚═══╝  ╚══════╝╚═╝  ╚═╝
    "#);
    println!("**************************************************************************************");

    let board = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |I|N|P|U|T|   |B|O|A|R|D|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    print_board(board);
    println!("**************************************************************************************");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |O|U|T|P|U|T|   |B|O|A|R|D|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");

    println!(" ");

    println!("************************ Solution - Parallel Version - Rayon ************************");

    let solutions = solve_par_rayon(board);
    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution {} - Parallel Rayon:\n{}", i + 1, format_board(solution));
        }
    }

    println!("**************************************************************************************");

    println!(" ");

    println!("************************ Solution - Parallel Version - Crossbeam ************************");

    let solutions = solve_par_crossbeam(board);

    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution - Parallel Crossbeam {}:\n{}", i + 1, format_board(solution));
        }
    }

    println!("**************************************************************************************");

    println!("*************************** Solution - Sequential Version 1 ***************************");
    let mut solver = crate::sequential_solver::SudokuSolver::new(board);
    let now = Instant::now();
    if solver.solve() {
        println!("Solution - Sequential :\n{}", solver);
    } else {
        println!("No solution exists.");
    }

    println!("**************************************************************************************");

    println!("*************************** Solution - Sequential Version 2 ***************************");
    let now = Instant::now();
    let solutions = crate::solver::sequential_1::solve_seq(board);
    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution - Sequential  {}:\n{}", i + 1, format_board(solution));
        }
    }

    println!("**************************************************************************************");

    let elapsed_time_rayon = Arc::new(Mutex::new(None));
    let elapsed_time_crossbeam = Arc::new(Mutex::new(None));
    let elapsed_time_seq1 = Arc::new(Mutex::new(None));
    let elapsed_time_seq2 = Arc::new(Mutex::new(None));

    let elapsed_rayon = Arc::clone(&elapsed_time_rayon);
    let elapsed_crossbeam = Arc::clone(&elapsed_time_crossbeam);
    let elapsed_seq1 = Arc::clone(&elapsed_time_seq1);
    let elapsed_seq2 = Arc::clone(&elapsed_time_seq2);

    // Measure time concurrently
    let _start_time = Instant::now();

    let handle_rayon = thread::spawn({
        let elapsed_rayon = Arc::clone(&elapsed_rayon);
        move || {
            let start = Instant::now();
            let _ = solve_par_rayon(board);
            *elapsed_rayon.lock().unwrap() = Some(start.elapsed().as_secs_f64());
        }
    });

    let handle_crossbeam = thread::spawn({
        let elapsed_crossbeam = Arc::clone(&elapsed_crossbeam);
        move || {
            let start = Instant::now();
            let _ = solve_par_crossbeam(board);
            *elapsed_crossbeam.lock().unwrap() = Some(start.elapsed().as_secs_f64());
        }
    });

    let handle_seq1 = thread::spawn({
        let elapsed_seq1 = Arc::clone(&elapsed_seq1);
        move || {
            let start = Instant::now();
            let mut solver = crate::sequential_solver::SudokuSolver::new(board);
            let _ = solver.solve();
            *elapsed_seq1.lock().unwrap() = Some(start.elapsed().as_secs_f64());
        }
    });

    let handle_seq2 = thread::spawn({
        let elapsed_seq2 = Arc::clone(&elapsed_seq2);
        move || {
            let start = Instant::now();
            let _ = solver::sequential_1::solve_seq(board);
            *elapsed_seq2.lock().unwrap() = Some(start.elapsed().as_secs_f64());
        }
    });
    handle_rayon.join().unwrap();
    handle_crossbeam.join().unwrap();
    handle_seq1.join().unwrap();
    handle_seq2.join().unwrap();

    // Print results
    println!("Elapsed time (parallel using rayon): {:.2?}", elapsed_time_rayon.lock().unwrap().unwrap_or(0.0));
    println!("Elapsed time (parallel using crossbeam): {:.2?}", elapsed_time_crossbeam.lock().unwrap().unwrap_or(0.0));
    println!("Elapsed time (sequential version 1): {:.2?}", elapsed_time_seq1.lock().unwrap().unwrap_or(0.0));
    println!("Elapsed time (sequential version 2): {:.2?}", elapsed_time_seq2.lock().unwrap().unwrap_or(0.0));;

}
