// TODO: Make crossover more general and easier to read, (interbreed with luck after truncate?, for a specific count or till a specific count, and replace mode or add mode) <<<
// TODO: Add doc stuff
// TODO: Complete the doc
// TODO: column creation optimization by using permutation mode (1..N)
// TODO: Terminal table TUI
// TODO: show all 3 chess boards of best of current generation, worst ever and best yet
// TODO: WASM interface?
// TODO: GUI mode
// TODO: cut half crossover rather than 0.5 chance
// TODO: debug improve
// TODO: more stat stuff like mean and mod of gen
// TODO: make evolve generator 
// TODO: Use faster DS
// TODO: async and parallelify with famous crates of rust for those stuff

use std::{time::Duration, vec};
use enum_display::EnumDisplay;

#[derive(Clone)]
pub struct ChessBoard {
    /// queens_mask[i] is the column of the i-th queen, in GA jargon, this array is our chromosome and each value is a genome
    queens_mask: Vec<usize>, 
    dim: usize,
}

// pub fn paste_vertical(strings: Vec<String>) {
//     let mut out: String = "".to_string();
//     let mut substrings_of_strings = Vec::<Vec::<String>>::with_capacity(strings.len());
//     let mut string_max_line_lenght = vec![0 as usize; strings.len()];
//     let mut max_substring_lenght = 0 as usize;
//     for string in strings {
//         substrings_of_strings.push(string.split('\n').map(|(s)| s.to_string()).collect());
//     }
//     /// make substrings count equal
//     for (i, substrings) in substrings_of_strings.iter().enumerate() {
//         string_max_line_lenght[i] = substrings.iter().max_by_key(|s| s.len()).unwrap().len();
//         if substrings.len() > max_substring_lenght {
//             max_substring_lenght = substrings.len();
//         }
//     }
//     for i in 0..max_substring_lenght {
//         out += substrings_of_strings[i]
//             .iter()
//             .enumerate()
//             .map(|(i, s)| s.clone() + &" ".to_string().repeat(string_max_line_lenght[i]-s.len()) + " ")
//             .collect::<String>();
//     }
// }

impl ChessBoard {
    pub fn fitness(&self, debug: bool) -> usize {
        let mut fitness: usize = 0;
        
        let mut masks_diags = vec![0 as usize ; (self.dim*2)-1];
        let mut mask_column = vec![0 as usize ; self.dim];
        
        for v in self.queens_mask.iter() {
            let index = v-1;
            mask_column[index] += 1;
            fitness += mask_column[index]-1
        }
        for (i, v) in self.queens_mask.iter().enumerate() {
            let index = i+(v-1);
            masks_diags[index] += 1;
            fitness += masks_diags[index]-1
        }
        masks_diags.fill(0);
        for (i, v) in self.queens_mask.iter().enumerate() {
            let index = (v-1)+(self.dim-1-i);
            masks_diags[index] += 1;
            fitness += masks_diags[index]-1;
        }
        if debug {
            println!("Fitness: {}", fitness);
            println!();
        }
        fitness
    }
    pub fn new_random(dim: usize) -> Self {
        let queens_mask = vec![(rand::random::<usize>() % dim) + 1; dim];
        Self { queens_mask, dim }
    }
    pub fn show(&self, mode: ShowingBoardModes) {
        match mode {
            ShowingBoardModes::Full => {
                for i in 0..self.dim {
                    for j in 0..self.dim {
                        if self.queens_mask[i] == j + 1 {
                            print!("Q ");
                        } else {
                            print!("_ ");
                        }
                    }
                    println!();
                }
            }
            ShowingBoardModes::Compact => {
                for i in 0..self.dim {
                    print!(
                        "{}{}",
                        self.queens_mask[i],
                        if i != self.dim - 1 { ", " } else { "" }
                    );
                }
                println!();
            }
            ShowingBoardModes::TUI => {
                todo!();
                // ╯ ╭╮│♛╰─
            }
            ShowingBoardModes::GUI => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, clap::ValueEnum)]
pub enum ShowingBoardModes {
    GUI,
    TUI,
    Full,
    Compact,
}

pub enum MutationStrategy {
    SliceRandom,
    SliceTendencyToEqality,
    Random
}

fn crossover(this: &ChessBoard, other: &ChessBoard) -> Result<ChessBoard, ()> {
    if this.dim != other.dim {
        return Err(());
    }
    let dim = this.dim;
    let mut queens_mask: Vec<usize> = Vec::with_capacity(this.dim);
    for i in 0..this.dim {
        if rand::random::<f64>() < 0.5 {
            queens_mask.push(this.queens_mask[i]);
        } else {
            queens_mask.push(other.queens_mask[i]);
        }
    }
    Ok(ChessBoard { queens_mask, dim })
}

fn mutate(this: &mut ChessBoard, strategy: MutationStrategy) {
    let mutation_rate: f64 = 0.08;
    for i in 0..this.dim {
        if rand::random::<f64>() < mutation_rate {
            this.queens_mask[i] = (rand::random::<usize>() % this.dim) + 1;
        }
    }
}

pub fn evolve(dim: usize, population_size: usize, debug: bool, time: Duration) -> Result<ChessBoard, ()>  {
    let mut population: Vec<ChessBoard> = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        population.push(ChessBoard::new_random(dim)); 
    }
    let mut best_fitness_yet: ChessBoard = population[0].clone(); 
    let mut worst_fitness_yet: ChessBoard = population[0].clone();

    loop {
        if population.len() == 2 {
            if debug {
                println!("failed!!!");
            }
            return Result::Err(());
        }
        population.sort_by_key(|board_a| board_a.fitness(false)); // improve this with max heap

        population.truncate(population.len() / 2); // if odd problem

        for i in 0..(population.len() - 2) {
            population.push(crossover(&population[i],&population[i + 1]).unwrap())
        }
        for board in &mut population {
            mutate(board, MutationStrategy::Random);
        }

        if population[0].fitness(false) < best_fitness_yet.fitness(false) {
            best_fitness_yet = population[0].clone() ;
        }
        if population[population.len() - 1].fitness(false) > worst_fitness_yet.fitness(false) {
            worst_fitness_yet = population[population.len() - 1].clone() ;
        }

        if debug {
            println!(
                "best of generation: {:#?}, worst of generation: {:#?}",
                population[0].fitness(false),
                population[population.len() - 1].fitness(false)
            );
            println!(
                "best of generation: {:#?}, worst of generation: {:#?}",
                population[0].fitness(false),
                population[population.len() - 1].fitness(false)
            );
            println!("current population: {}", population.len());
            population[0].show(ShowingBoardModes::Full);
            population[0].show(ShowingBoardModes::Compact);
            println!("{}", population.len());
        }
        if population[0].fitness(false) == 0 {
            if debug {
                println!("BINGO, WE FOUND AN ANSWER!!!");
            }
            return Result::Ok(population[0].clone());
        }
        std::thread::sleep(time);
    }
}