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

// Run a benchmark with input from a file. Each line in a file contains the sequence of moves
// and the expected score the engine should evaluate to
// Outputs the average time taken to solve position, avg number of nodes searched, and avg node search rate.
fn benchmark(file: &str, per_case_output: bool) {
    println!("Running benchmark: {}", file);
    let mut times = Vec::new();
    let mut nodes_searched = Vec::new();
    let lines = read_lines(file);
    let mut n_correct = 0;

    for line in lines {
        let mut splits = line.split(' ');
        let moves = splits.next().unwrap();
        let expected_score = splits.next().unwrap().parse::<i32>().unwrap();
        let board = Board::from_notation(moves);

        let now = std::time::Instant::now();
        let result = negamax(&board);
        let elapsed = now.elapsed().as_micros();
        times.push(elapsed);
        nodes_searched.push(result.nodes_searched);
        if result.score == expected_score {
            n_correct += 1;
        }

        if per_case_output {
            println!(
                "Game #{}: {} - {}us - {} nodes - {} Kpos/s",
                times.len(),
                if result.score == expected_score {
                    "PASSED"
                } else {
                    "FAILED"
                },
                elapsed,
                result.nodes_searched,
                result.nodes_searched as f32 / elapsed as f32 * 1_000.0
            );
        }
    }

    println!("Benchmark result: {}", file);
    println!("Number of entries: {}", times.len());
    println!(
        "Number of correct scores: {} ({:.2}%)",
        n_correct,
        n_correct as f32 / times.len() as f32 * 100.0
    );
    println!(
        "Average time taken: {}μs",
        times.iter().sum::<u128>() as f32 / times.len() as f32
    );
    println!(
        "Average nodes searched: {}",
        nodes_searched.iter().sum::<usize>() as f32 / nodes_searched.len() as f32
    );
    println!(
        "Average nodes searched per second: {} Kpos/s",
        nodes_searched.iter().sum::<usize>() as f32 / times.iter().sum::<u128>() as f32 * 1_000.0
    );
}

fn main() {
    benchmark("benchmarks/Test_L3_R1.txt", false); // End game - Easy
    println!("==============================");
    //benchmark("benchmarks/Test_L2_R1.txt", true);  // Mid game - Easy
}
