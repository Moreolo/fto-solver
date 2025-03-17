use std::{fs, time::Instant};

use bytemuck::cast_slice;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{iter::{IntoParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

use crate::pruning::format_duration;

use super::State;

pub trait SymHash: State {
    const SYM_NAME: &str;
    const SYM_SIZE: usize;

    fn get_symhash_table() -> Vec<usize> {
        match Self::load_table() {
            Ok(table) => table,
            Err(_) => {
                Self::generate_table()
            }
        }
    }

    fn from_sym_hash(table: &Vec<usize>, sym_hash: usize) -> Self {
        Self::from_hash(table[sym_hash])
    }

    fn get_syms(&self) -> Vec<u8> {
        let hash = self.get_hash();
        self.get_sym_states().into_iter().enumerate().filter_map(|(sym, state)| if state.get_hash() == hash {Some(sym as u8)} else {None}).collect()
    }

    fn get_sym_hash(&self, table: &Vec<usize>) -> usize {
        Self::get_sym_hash_from_repr(table, self.get_repr())
    }

    fn get_repr(&self) -> usize {
        self.get_sym_states().iter().map(|state| state.get_hash()).min().unwrap()
    }

    fn get_sym_hash_from_repr(table: &Vec<usize>, repr: usize) -> usize {
        table.iter().position(|&other| repr == other).expect("Representant not in Array")
    }

    fn get_sym_hash_sym(&self, table: &Vec<usize>) -> (usize, u8) {
        let (repr, sym) = self.get_sym_states().into_iter().enumerate().map(|(sym, state)| (state.get_hash(), sym as u8)).min_by_key(|(state, _)| *state).unwrap();
        (Self::get_sym_hash_from_repr(table, repr), sym)
    }

    fn generate_table() -> Vec<usize> {
        println!("Generating table {}", Self::get_file_name());
        let now = Instant::now();
        let bar = ProgressBar::new(Self::RAW_SIZE as u64);
        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10}").unwrap().progress_chars("#>-"));
        let mut table: Vec<usize> = (0..Self::RAW_SIZE).into_par_iter().map(|raw_hash| {
            bar.inc(1);
            Self::from_hash(raw_hash).get_repr()
        }).collect();
        table.par_sort();
        table.dedup();
        let elapsed = now.elapsed();
        bar.finish();
        println!("Finished generating Table in {}", format_duration(elapsed));
        println!("Size: {}", table.len());
        Self::save_table(&table);
        table
    }

    fn save_table(table: &Vec<usize>) {
        match fs::write(Self::get_file_name(), cast_slice(table)) {
            Ok(_) => println!("Saved table {}", Self::get_file_name()),
            Err(_) => println!("Couldn't write table {}", Self::get_file_name())
        }
    }

    fn load_table() -> Result<Vec<usize>, std::io::Error> {
        match fs::read(Self::get_file_name()) {
            Ok(data) => Ok(cast_slice(&data).to_vec()),
            Err(err) => Err(err)
        }
    }

    fn get_file_name() -> String {
        format!("tables/symhash/{}.bin", Self::SYM_NAME)
    }
}