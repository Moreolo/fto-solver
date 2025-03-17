use std::{fs, time::Instant};

use bytemuck::cast_slice;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{pruning::format_duration, state::State};

// also: convert hash (or sym hash) to raw

pub trait SymTable: State + Clone {
    const SYMTABLE_NAME: &str;

    fn get_sym_table() -> Vec<usize> {
        match Self::load_table() {
            Ok(table) => table,
            Err(_) => {
                Self::generate_table()
            }
        }
    }

    fn apply_sym_table(table: &Vec<usize>, hash: usize, sym: u8) -> usize {
        table[hash * Self::NUM_SYMS as usize + sym as usize]
    }

    fn apply_syms_table(table: &Vec<usize>, hash: usize) -> Vec<usize> {
        table[hash * Self::NUM_SYMS as usize .. (hash + 1) * Self::NUM_SYMS as usize].to_vec()
    }

    fn generate_table() -> Vec<usize> {
        println!("Generating table {}", Self::get_file_name());
        let now = Instant::now();
        let bar = ProgressBar::new(Self::RAW_SIZE as u64);
        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10}").unwrap().progress_chars("#>-"));
        let table: Vec<usize> = (0..Self::RAW_SIZE).into_par_iter().flat_map_iter(|hash| {
            bar.inc(1);
            let state = Self::from_hash(hash);
            (0..Self::NUM_SYMS).map(move |sym| {
                let mut adj = state.clone();
                adj.wrap_sym(sym);
                adj.get_hash()
            })
        }).collect();
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
        format!("tables/sym/{}.bin", Self::SYMTABLE_NAME)
    }
}