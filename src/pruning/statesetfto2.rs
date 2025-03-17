use crate::{moving::turn::TurnTable, state::{statefto2::{StateFTO2, FTO2_SYMHASH_TABLE, FTO2_TURN_TABLE}, symhash::SymHash, State}};

use super::stateset::StateSet;


pub struct StateSetFTO2 {
    fto2: usize
}

impl StateSet for StateSetFTO2 {
    const NAME: &str = "fto2";

    const SIZE: usize = StateFTO2::SYM_SIZE;

    const MAX_DEPTH: u8 = 10;

    fn solved() -> Self {
        Self {
            fto2: StateFTO2::solved().get_sym_hash(&FTO2_SYMHASH_TABLE)
        }
    }

    fn from_hash(hash: usize) -> Self {
        Self {
            fto2: hash
        }
    }

    fn get_hash(&self) -> usize {
        self.fto2
    }

    fn get_sym_hashes(&self) -> Vec<usize> {
        vec![]
    }

    fn get_next_state_sets(&self) -> Vec<Self> where Self: Sized {
        StateFTO2::apply_turns_table(&FTO2_TURN_TABLE, self.fto2).iter().map(|&hash| Self{fto2: hash / StateFTO2::NUM_SYMS as usize}).collect()
        // StateFTO2::from_sym_hash(&FTO2_SYMHASH_TABLE, self.fto2).get_next_states().iter().map(|fto2| Self {fto2: fto2.get_sym_hash(&FTO2_SYMHASH_TABLE)}).collect()
    }
}