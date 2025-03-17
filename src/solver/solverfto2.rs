use crate::{pruning::{statesetfto2::StateSetFTO2, PruningTable}, puzzle::Puzzle, state::{statefto2::{StateFTO2, FTO2_SYMHASH_TABLE}, symhash::SymHash, State}};


pub struct SolverFTO2 {
    pruning_table: Vec<u8>
}

impl SolverFTO2 {
    pub fn new() -> Self {
        Self {
            pruning_table: match PruningTable::<StateSetFTO2>::read_table_from_file() {
                Ok(table) => table,
                Err(_) => PruningTable::<StateSetFTO2>::new(true).generate(true)
            }
        }
    }

    pub fn solve(&self, fto2: &StateFTO2) -> String {
        let mut solution = String::new();
        let mut current = fto2.clone();
        let mut depthm3 = self.get_depthm3(current.get_sym_hash(&FTO2_SYMHASH_TABLE));
        let mut changed = true;
        while changed {
            changed = false;
            for turn in 0..8 {
                let mut potential = current.clone();
                potential.apply_turn(turn);
                let potential_depthm3 = self.get_depthm3(potential.get_sym_hash(&FTO2_SYMHASH_TABLE));
                if (potential_depthm3 + 1) % 3 == depthm3 {
                    solution += StateFTO2::get_notation(turn);
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
        PruningTable::<StateSetFTO2>::read(&self.pruning_table, hash)
    }
}