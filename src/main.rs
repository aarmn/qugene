use std::time::Duration;

#[derive(Clone)]
struct ChessBoard<const N: usize> {
    queens_mask: [usize; N], // the chromosome, each one is a gene
}

impl<const N: usize> ChessBoard<N> {
    fn fitness(&self, debug: bool) -> usize {
        let mut fitness: usize = 0;
        for i in 0..N {
            for j in (i + 1)..N {
                if debug {
                    println!(
                        "i:{} j:{} queens_mask[i]:{} queens_mask[j]:{}",
                        i, j, self.queens_mask[i], self.queens_mask[j]
                    );
                    println!();
                }
                fitness += Into::<usize>::into(self.queens_mask[i] == self.queens_mask[j]);
            }
        }
        for i in 0..N {
            for j in (i + 1)..N {
                if debug {
                    println!(
                        "i:{} j:{} queens_mask[i]:{} queens_mask[j]:{} (j-i):{}",
                        i,
                        j,
                        self.queens_mask[i],
                        self.queens_mask[j],
                        j - i
                    );
                    println!();
                }
                fitness += Into::<usize>::into(self.queens_mask[i] + (j - i) == self.queens_mask[j]);
            }
        }
        for i in (0..N).rev() {
            for j in ((i + 1)..N).rev() {
                if debug {
                    println!(
                        "i:{} j:{} queens_mask[i]:{} queens_mask[j]:{} (j-i):{}",
                        i,
                        j,
                        self.queens_mask[i],
                        self.queens_mask[j],
                        j - i
                    );
                    println!();
                }
                let v1: i64 = self.queens_mask[i] as i64 - (j as i64 - i as i64);
                let v2: i64 = (self.queens_mask[j]).try_into().unwrap();
                fitness += Into::<usize>::into(v1 == v2);
            }
        }
        fitness
    }
    fn new_random() -> Self {
        let mut queens_mask: [usize; N] = [0; N];
        for i in 0..N {
            queens_mask[i] = (rand::random::<usize>() % N) + 1;
        }
        Self { queens_mask }
    }
    fn crossover(&self, other: &Self) -> Self {
        let mut queens_mask: [usize; N] = [0; N];
        for i in 0..N {
            queens_mask[i] = if rand::random::<f64>() < 0.5 {
                self.queens_mask[i]
            } else {
                other.queens_mask[i]
            };
        }
        Self { queens_mask }
    }
    fn mutate(&mut self) {
        const MUTATION_RATE: f64 = 0.08;
        for i in 0..N {
            if rand::random::<f64>() < MUTATION_RATE {
                self.queens_mask[i] = (rand::random::<usize>() % N) + 1;
            }
        }
    }
    fn show(&self, mode: ShowingBoardModes) {
        match mode {
            ShowingBoardModes::Full => {
                for i in 0..N {
                    for j in 0..N {
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
                for i in 0..N {
                    print!(
                        "{}{}",
                        self.queens_mask[i],
                        if i != N - 1 { ", " } else { "" }
                    );
                }
                println!();
            }
            ShowingBoardModes::TUI => {
                todo!();
                // let n = self.queens_mask.len();
                // println!("╭{}╮", "─".repeat(n * 2 - 1));
                // for (i, item) in self.queens_mask.iter().enumerate() {
                //     println!("│{}│", if "♛".repeat(item).trim_end());
                //     println!("{}", "─".repeat(n * 2 - 1));
                // }
                // println!("╰{}╯", "─".repeat(n * 2 - 1));
            }
            ShowingBoardModes::GUI => {
                todo!()
            }
        }
    }
}

enum ShowingBoardModes {
    GUI,
    TUI,
    Full,
    Compact,
}

fn evolve<const N: usize>(population_size: usize, debug: bool, time: Duration) -> Result<ChessBoard<N>, ()>  {
    let mut population: Vec<ChessBoard<N>> = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        population.push(ChessBoard::new_random()); // permutation of 1..N
    }
    let mut best_fitness: usize = usize::MAX; // keep board as well
    let mut worst_fitness: usize = usize::MAX;
    // yield
    // population[0].show(ShowingBoardModes::TUI);
    // async and parallel

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
            population.push(population[i].crossover(&population[i + 1]))
        }
        for board in &mut population {
            board.mutate()
        }

        if population[0].fitness(false) < best_fitness {
            best_fitness = population[0].fitness(false) ;
        }
        if population[population.len() - 1].fitness(false) > worst_fitness {
            worst_fitness = population[population.len() - 1].fitness(false) ;
        }

        if debug {
            println!(
                "best of population: {:#?}, worst of population: {:#?}",
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
                println!("BINGO!!!");
            }
            return Result::Ok(population[0].clone());
        }
        std::thread::sleep(time);
        // commandline interface for speed and showing mode[s]
    }
}

fn main() { // result
    // time it
    evolve::<8>(700,true, Duration::from_micros(10)).unwrap().show(ShowingBoardModes::Compact);
}

// interbreed with luck after truncate?, for a specific count or till a specific count, and replace mode or add mode
