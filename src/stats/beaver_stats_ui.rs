use colored::Colorize;

use crate::stats::utils::BeaverType;
use crate::stats::beaver_stats::BeaverStats;
use crate::stats::utils::StatesAndSymbols;
use crate::stats::aggregate_stat::AggregateStat;
use crate::stats::competitive_stat::CompetitiveStat;
use crate::stats::utils::Stat;

impl BeaverStats {
    pub fn display_tally(&self, states_and_symbols: StatesAndSymbols) {
        println!(
            "Tallies of beavers with {} states and {} symbols:",
            states_and_symbols.n_states,
            states_and_symbols.n_symbols,
        );
        println!("undetermined: {}", self.types.get(&BeaverType::Undetermined).unwrap().get_tally(states_and_symbols));
    }

    pub fn display_list(&self, states_and_symbols: StatesAndSymbols, stat: Stat, score: u64) {
        println!(
            "List of beavers with {} states and {} symbols that have a {} of {}:\n",
            states_and_symbols.n_states,
            states_and_symbols.n_symbols,
            stat.to_string(),
            score,
        );

        let competitive_stat = match stat {
            Stat::Halting(halting_stat) => self.halting.get(&halting_stat).unwrap(),
            Stat::Infinite(infinite_stat) => self.infinite.get(&infinite_stat).unwrap(),
        };

        let histogram = match competitive_stat.histograms.get(&states_and_symbols) {
            Some(histogram) => histogram,
            None => {
                println!("No beavers yet!");
                return;
            }
        };

        let beavers = match histogram.get(score as usize) {
            Some(beavers) => beavers,
            None => {
                println!("No beavers yet!");
                return;
            }
        };

        for beaver in beavers {
            println!("{}", beaver);
        }
    }

    pub fn display_histogram(&self, stat: Stat, states_and_symbols: StatesAndSymbols) {
        match stat {
            Stat::Halting(halting_stat) => self.halting.get(&halting_stat).unwrap().display_histogram(states_and_symbols),
            Stat::Infinite(infinite_stat) => self.infinite.get(&infinite_stat).unwrap().display_histogram(states_and_symbols),
        }
    }
}

impl CompetitiveStat {
    const MAX_HISTOGRAM_BLOCKS: usize = 100;

    pub fn display_histogram(&self, states_and_symbols: StatesAndSymbols) {
        let histogram = self.histograms.get(&states_and_symbols).unwrap();
        let start_index = histogram.iter().position(|beavers| !beavers.is_empty()).unwrap();
        let highest_count = histogram.iter().map(|beavers| beavers.len()).max().unwrap();

        for (score, beaver_ids) in histogram.iter().enumerate().take(Self::STAT_LIST_LENGTH).skip(start_index) {
            let num_blocks = (beaver_ids.len() as f64 / highest_count as f64) as usize * Self::MAX_HISTOGRAM_BLOCKS;
            print!("{:02} | ", score);
            print!("{} | ", "█".repeat(num_blocks).blue());
            println!("{}", beaver_ids.len());
        } 
    } 

    pub fn display_leaderboard(&self, states_and_symbols: StatesAndSymbols) {
        println!(
            "Leaderboard for beavers with {} states and {} symbols",
            states_and_symbols.n_states,
            states_and_symbols.n_symbols
        );

        let leaderboard = self.leaderboards.get(&states_and_symbols).unwrap();
        for (rank, beaver_score) in leaderboard.iter().enumerate() {
            println!("{:02}. {} | {}", rank + 1, beaver_score.beaver, beaver_score.score);
        }
    }
}

impl AggregateStat {
    pub fn get_tally(&self, states_and_symbols: StatesAndSymbols) -> u64 {
        *self.tallies.get(&states_and_symbols).unwrap_or(&0)
    }
}