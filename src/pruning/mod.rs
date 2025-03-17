
use std::{fs, str::FromStr, sync::{Arc, RwLock}, time::{Duration, Instant}};

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub mod stateset;
pub mod statesetfto2;
pub mod statesetfto2split;

pub mod statesetpyra;

use stateset::StateSet;

pub struct PruningTable <S: StateSet + Sync + Send> {
    pb_table: ProgressBar,
    pb_closed: ProgressBar,
    _marker: std::marker::PhantomData<S>
}

impl<S: StateSet + Sync + Send> PruningTable<S> {
    pub fn new(display_progress: bool) -> Self {
        let _multipb = if display_progress {MultiProgress::new()} else {MultiProgress::with_draw_target(ProgressDrawTarget::hidden())};
        let pb_table = _multipb.add(ProgressBar::new(S::SIZE as u64));
        let pb_closed = _multipb.add(ProgressBar::new(1));

        let style_table: ProgressStyle = ProgressStyle::with_template(
            "[{elapsed_precise}] Table: {bar:40.cyan/blue} {percent_precise:>7}% {msg}"
        )
        .unwrap()
        .progress_chars("#>-");
        let style_opened = ProgressStyle::with_template(
            "Pruning Depth {msg:>2}: {bar:40.gray/white} {percent_precise:>7}%"
        )
        .unwrap()
        .progress_chars("##-");

        pb_table.set_style(style_table);
        pb_closed.set_style(style_opened);
        pb_closed.set_message(format!("{}", 0));

        Self {
            pb_table,
            pb_closed,
            _marker: std::marker::PhantomData
        }
    }

    pub fn generate(&self, fill_last: bool) -> Vec<u8> {
        // Starts time measurement
        let now = Instant::now();

        // Creates empty Pruning Table
        let shared_table: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(vec![255 as u8; (S::SIZE + 3) / 4]));

        // Creates empty closed Table
        let mut closed = vec![];

        // Fills in the solved State and adds first closed State
        let state = S::solved();
        closed.push(state.get_hash());
        if Self::write_shared(&shared_table, state.get_hash(), 0).is_ok() {
            self.pb_table.inc(1);
        }
        for hash in state.get_sym_hashes() {
            if Self::write_shared(&shared_table, hash, 0).is_ok() {
                self.pb_table.inc(1);
            }
        }

        // Starts looping over the Pruning Depths
        let mut pruning_depth = 1;
        while !closed.is_empty() && !self.table_is_full() {
            // Shows Progress
            self.clear_pb_closed(closed.len() as u64, pruning_depth);

            // Iterates over all States in closed Table
            let depthm3 = pruning_depth % 3;
            let at_max = pruning_depth == S::MAX_DEPTH - 1 && fill_last;
            closed = closed.into_par_iter().flat_map_iter(|curr_state_set| {
                // Shows Progress
                self.pb_closed.inc(1);

                // Opens the next States
                S::from_hash(curr_state_set).get_next_state_sets().into_iter().filter_map(|next_state_set| {
                    let hash = next_state_set.get_hash();
                    // Tries to write State to Table
                    match Self::write_shared(&shared_table, hash, depthm3) {
                        Ok(_) => {
                            // On write, also write symmetric States
                            let inc = 1 + next_state_set.get_sym_hashes().into_iter().filter(|&sym_hash| {
                                Self::write_shared(&shared_table, sym_hash, depthm3) == Ok(())
                            }).count();
                            // Increase Progressbar
                            self.pb_table.inc(inc as u64);
                            if !at_max {
                                Some(hash)
                            } else {
                                None
                            }
                        }
                        Err(_) => None
                    }
                })
            }).collect();
            // Increases the Pruning Depth
            pruning_depth += 1;
        }

        // Fills rest of the Table
        if pruning_depth == S::MAX_DEPTH && !self.table_is_full() && fill_last {
            let depthm3 = pruning_depth % 3;
            // Shows Progress
            self.pb_table.set_message("Filling rest");
            self.clear_pb_closed(S::SIZE as u64 - self.pb_table.position(), pruning_depth);

            // Iterates Table
            let mut table = shared_table.write().unwrap();
            table.par_iter_mut().for_each(|table_value| {
                let mut changed: u64 = 0;
                // Fills empty entries
                for entry_index in 0..4 {
                    let true_entry_index = entry_index << 1;
                    if (*table_value >> true_entry_index) & 3 == 3 {
                        *table_value = (depthm3 << true_entry_index) + (*table_value & !(3 << true_entry_index));
                        changed += 1;
                    }
                };
                // Shows progress
                self.pb_table.inc(changed);
                self.pb_closed.inc(changed);
            });
        }

        // Finishes Time measurement
        let elapsed = now.elapsed();
        // Completes Progress
        let filled = self.table_is_full();
        let fill_level = self.pb_table.position();
        self.pb_table.finish();
        self.pb_closed.finish();
        println!("Finished generating Table in {}", format_duration(elapsed));
        if !filled {
            println!("Table not full: {}/{}", fill_level, self.pb_table.length().unwrap());
        }

        // Saves Table to file
        println!("Saving Table to file");
        let table = shared_table.read().unwrap();
        fs::write(Self::get_file_name(), table.clone()).expect("Saving Table failed!");
        table.clone()
    }

    fn table_is_full(&self) -> bool {
        self.pb_table.position() >= S::SIZE as u64
    }

    // Resets the Progressbar for the closed Table
    fn clear_pb_closed(&self, closed_len: u64, slice_depth: u8) {
        self.pb_closed.set_position(0);
        self.pb_closed.set_length(closed_len);
        self.pb_closed.set_message(format!("{}", slice_depth));
    }

    // Writes a value into the index of the shared Table
    fn write_shared(shared_table: &Arc<RwLock<Vec<u8>>>, index: usize, value: u8) -> Result<(), ()> {
        let entry_index = (index & 3) << 1;
        if {
            // Gets Lock
            let table = shared_table.read().unwrap();
            // Checks, if entry is already filled
            let table_value = table[index >> 2];
            (table_value >> entry_index) & 3 == 3
        } {
            // Gets Lock
            let mut table = shared_table.write().unwrap();
            // Gets entry
            let table_value = table[index >> 2];
            // Writes new u2 entry onto u8 slot
            if (table_value >> entry_index) & 3 == 3 {
                table[index >> 2] = (value << entry_index) + (table_value & !(3 << entry_index));
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err (())
        }
    }

    pub fn read(table: &Vec<u8>, index: usize) -> u8 {
        (table[index >> 2] >> ((index & 3) << 1)) & 3
    }

    pub fn read_table_from_file() -> Result<Vec<u8>, std::io::Error> {
        fs::read(Self::get_file_name())
    }

    pub fn get_file_name() -> String {
        String::from_str("tables/pruning/").unwrap() + S::NAME + ".bin"
    }
}

pub fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs();
    let minutes = secs / 60;
    if minutes > 60 {
        format!("{}h {:2}min", minutes / 60, minutes & 60)
    } else if minutes > 0 {
        format!("{}min {:2}s", minutes, secs & 60)
    } else {
        format!("{:.2?}", dur)
    }
    // 1h 45min 34s
}