mod evolution;

use std::time::{Duration, Instant};
use crate::evolution::{evolve, ShowingBoardModes, ChessBoard};
use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(name = "qugene", about = "A simple GA to solve N Queens problem", version)]
#[clap(author = "AARMN The Limitless <aarmn80@gmail.com> <github.com/aarmn/qugene>")]
struct Opts {
    /// Activate verbose mode to show more information
    #[clap(short, long)]
    verbose: bool,
    
    /// Just shows the output when done
    #[clap(short, long, conflicts_with = "verbose")]
    quite: bool,

    /// Sets the population size
    #[clap(short, long, default_value = "500")]
    population: usize,

    /// Independent rate of mutation per each gene
    #[clap(short, long, default_value = "0.08")]
    mutation_rate: f64,

    /// Sets the board view mode
    #[clap(short, long)]
    #[clap(value_enum, default_value_t=ShowingBoardModes::Full)]
    chessboard_view: ShowingBoardModes,

    // /// Sets the time limit in microseconds
    // #[clap(short, long, default_value = None)]
    // time_limit: Option<u64>,

    /// Sets the number of times to rerun the algorithm
    #[clap(short, long, default_value = None)]
    rerun_limit: Option<u64>,

    /// Sets the board n for n-queen problem
    #[clap(required = true)]
    size: usize,

    // /// Sets the optimization mode to don't allow creation of queens in the same column (experimental)
    // #[clap(short, long)]
    // optimization_columns: bool
}

fn main() {
    let args = Opts::parse();
    let start: Instant = Instant::now();
    let mut got_result: bool = false;
    for _ in 0..args.rerun_limit.unwrap_or(u64::MAX) {
        let result = evolve(args.size, args.population,args.verbose, Duration::from_micros(0));
        if let Ok(done)=result {
            println!("\n\n Final Board:\n\n");
            done.show(args.chessboard_view);
            got_result = true;
            break;
        }
    } 
    if !got_result {
        println!("\n\nNo solution found :(\n\n");
    }
    let duration = start.elapsed();
    println!("Time elapsed in seconds: {:.5}", duration.as_secs_f64());
}
// .color(clap::ColorChoice::Always)