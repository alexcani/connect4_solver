// Purpose: Benchmarks for the project.
// Run with --release to get accurate results.
use connect4_solver::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect()
}

struct CaseResult {
    correct: bool,
    time_taken_ns: u128,
    nodes_searched: usize,
}

fn format_time_ns(ns: u128) -> String {
    let ns = ns as f32;
    if ns < 1_000.0 {
        format!("{}ns", ns)
    } else if ns < 1_000_000.0 {
        format!("{:.2}us", ns / 1_000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.2}ms", ns / 1_000_000.0)
    } else {
        format!("{:.2}s", ns / 1_000_000_000.0)
    }
}

// Run a benchmark with input from a file. Each line in a file contains the sequence of moves
// and the expected score the engine should evaluate to
// Outputs the average time taken to solve position, avg number of nodes searched, and avg node search rate.
fn benchmark<C, S, B>(file: &str, board_creator: C, mut solver: S, per_case_output: bool)
where
    C: Fn(&str) -> B + Sync,
    B: Board,
    S: FnMut(&B) -> SolveResult + Sync,
{
    println!("Running benchmark: {}", file);
    let now = std::time::Instant::now();
    let lines = read_lines(file);
    let results = lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let mut splits = line.split(' ');
            let moves = splits.next().unwrap();
            let expected_score = splits.next().unwrap().parse::<i32>().unwrap();
            let board = board_creator(moves);

            let now = std::time::Instant::now();
            let result = solver(&board);
            let elapsed = now.elapsed().as_nanos();

            let result = CaseResult {
                correct: result.score == expected_score,
                time_taken_ns: elapsed,
                nodes_searched: result.nodes_searched,
            };

            if per_case_output {
                println!(
                    "Game #{}: {} - {}us - {} nodes - {} Kpos/s",
                    index,
                    if result.correct { "PASSED" } else { "FAILED" },
                    elapsed,
                    result.nodes_searched,
                    result.nodes_searched as f32 / elapsed as f32 * 1_000.0
                );
            }

            result
        })
        .collect::<Vec<_>>();
    let elapsed = now.elapsed().as_nanos();

    println!("Benchmark result: {}", file);
    println!("Time taken: {}", format_time_ns(elapsed));
    println!("Number of entries: {}", results.len());
    println!(
        "Number of correct scores: {} ({:.2}%)",
        results.iter().filter(|r| r.correct).count(),
        results.iter().filter(|r| r.correct).count() as f32 / results.len() as f32 * 100.0
    );
    println!(
        "Average time taken: {}",
        format_time_ns(
            results.iter().map(|r| r.time_taken_ns).sum::<u128>() / results.len() as u128
        )
    );
    println!(
        "Average nodes searched: {}",
        results.iter().map(|r| r.nodes_searched).sum::<usize>() as f32 / results.len() as f32
    );
    println!(
        "Average nodes searched per second: {} Kpos/s",
        results.iter().map(|r| r.nodes_searched).sum::<usize>() as f32
            / results.iter().map(|r| r.time_taken_ns).sum::<u128>() as f32
            * 1_000_000.0
    );
}

fn main() {
    println!("(ArrayBoard) NEGAMAX - NAIVE");
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        ArrayBoard::from_notation,
        negamax,
        false,
    ); // End game - Easy

    println!("==============================");

    println!("(ArrayBoard) NEGAMAX - ALPHA-BETA PRUNING");
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        ArrayBoard::from_notation,
        negamax_ab,
        false,
    ); // End game - Easy

    println!("==============================");

    println!("(BitBoard) NEGAMAX - NAIVE");
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        BitBoard::from_notation,
        negamax,
        false,
    ); // End game - Easy

    println!("==============================");

    println!("(BitBoard) NEGAMAX - ALPHA-BETA PRUNING");
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        BitBoard::from_notation,
        negamax_ab,
        false,
    ); // End game - Easy
    println!("----------------");
    benchmark(
        "benchmarks/Test_L2_R1.txt",
        BitBoard::from_notation,
        negamax_ab,
        false,
    ); // Mid game - Easy

    println!("==============================");

    // Use the same solver for all benchmarks since recreating the cache would defeat its purpose
    let mut solver = NegamaxSolver::new_with_table();

    println!("(BitBoard) NEGAMAX - ALPHA-BETA PRUNING - TRANSPOSITION TABLE");
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // End game - Easy
    println!("----------------");
    benchmark(
        "benchmarks/Test_L2_R1.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // Mid game - Easy
    println!("----------------");
    benchmark(
        "benchmarks/Test_L2_R2.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // Mid game - Medium

    println!("==============================");

    println!("(BitBoard) NEGAMAX - ITERATIVE DEEPENING WITH NULL WINDOW SEARCH");

    let mut solver = NegamaxID::new();
    benchmark(
        "benchmarks/Test_L3_R1.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // End game - Easy
    println!("----------------");
    benchmark(
        "benchmarks/Test_L2_R1.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // Mid game - Easy
    println!("----------------");
    benchmark(
        "benchmarks/Test_L2_R2.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // Mid game - Medium
    println!("----------------");
    benchmark(
        "benchmarks/Test_L1_R1.txt",
        BitBoard::from_notation,
        |board| solver.solve(board),
        false,
    ); // Early game - Easy
}
