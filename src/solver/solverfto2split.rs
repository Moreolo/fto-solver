use crate::{pruning::{stateset::StateSet, statesetfto2split::StateSetFTO2Split, PruningTable}, puzzle::Puzzle, state::{statefto2::StateFTO2, State}};


pub struct SolverFTO2Split {
    pruning_table: Vec<u8>
}

impl SolverFTO2Split {
    pub fn new() -> Self {
        Self {
            pruning_table: match PruningTable::<StateSetFTO2Split>::read_table_from_file() {
                Ok(table) => table,
                Err(_) => PruningTable::<StateSetFTO2Split>::new(true).generate(true)
            }
        }
    }

    pub fn solve(&self, fto2: &StateFTO2) -> String {
        let mut solution = String::new();
        let mut current = fto2.clone();
        let mut depthm3 = self.get_depthm3(&current);
        let mut changed = true;
        while changed {
            changed = false;
            for turn in 0..8 {
                let mut potential = current.clone();
                potential.apply_turn(turn);
                let potential_depthm3 = self.get_depthm3(&potential);
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

    fn get_depthm3(&self, fto2: &StateFTO2) -> u8 {
        let (cp, co, ce) = fto2.get_parts();
        let stateset = StateSetFTO2Split::from_parts(cp, co, ce);
        PruningTable::<StateSetFTO2Split>::read(&self.pruning_table, stateset.get_hash())
    }
}