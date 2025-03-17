use crate::{moving::turn::TurnTable, state::{statepyra::{StatePyra, PYRA_SYMHASH_TABLE, PYRA_TURN_TABLE}, symhash::SymHash, State}};

use super::stateset::StateSet;


pub struct StateSetPyra {
    pyra: usize
}

impl StateSet for StateSetPyra {
    const NAME: &str = "pyra";
    const SIZE: usize = StatePyra::SYM_SIZE;
    const MAX_DEPTH: u8 = 11;

    fn solved() -> Self {
        Self {
            pyra: StatePyra::solved().get_sym_hash(&PYRA_SYMHASH_TABLE)
        }
    }

    fn from_hash(hash: usize) -> Self {
        Self {
            pyra: hash
        }
    }

    fn get_hash(&self) -> usize {
        self.pyra
    }

    fn get_sym_hashes(&self) -> Vec<usize> {
        vec![]
    }

    fn get_next_state_sets(&self) -> Vec<Self> where Self: Sized {
        StatePyra::apply_turns_table(&PYRA_TURN_TABLE, self.pyra).iter().map(|&hash| Self{pyra: hash / StatePyra::NUM_SYMS as usize}).collect()
    }
}