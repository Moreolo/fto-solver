use crate::{pruning::{statesetpyra::StateSetPyra, PruningTable}, puzzle::Puzzle, state::{statefto::StateFTO, statepyra::{StatePyra, PYRA_SYMHASH_TABLE}, symhash::SymHash, State}};


pub struct SolverPyra {
    pruning_table: Vec<u8>
}

impl SolverPyra {
    pub fn new() -> Self {
        Self {
            pruning_table: match PruningTable::<StateSetPyra>::read_table_from_file() {
                Ok(table) => table,
                Err(_) => PruningTable::<StateSetPyra>::new(true).generate(true)
            }
        }
    }

    pub fn solve(&self, pyra: &StatePyra) -> String {
        let mut solution = String::new();
        let mut current = StateFTO::from_pyra(pyra);
        let mut depthm3 = self.get_depthm3(StatePyra::from_fto(&current).get_sym_hash(&PYRA_SYMHASH_TABLE));
        let mut changed = true;
        while changed {
            changed = false;
            for turn in 0..8 {
                let mut potential = current.clone();
                potential.apply_turn(turn);
                let potential_depthm3 = self.get_depthm3(StatePyra::from_fto(&potential).get_sym_hash(&PYRA_SYMHASH_TABLE));
                if (potential_depthm3 + 1) % 3 == depthm3 {
                    solution += StateFTO::get_notation(turn);
                    solution.push(' ');
                    current = potential;
                    depthm3 = potential_depthm3;
                    changed = true;
                    break;
                }
            }
        }
        solution.pop();
        solution
    }

    fn get_depthm3(&self, hash: usize) -> u8 {
        PruningTable::<StateSetPyra>::read(&self.pruning_table, hash)
    }
}