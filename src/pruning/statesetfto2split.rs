use crate::{moving::{sym::SymTable, turn::TurnTable}, state::{statecenters::{StateCenters, CENTERS_SYM_TABLE, CENTERS_TURN_TABLE}, statecorners::{StateCorners, CORNERS_SYMHASH_TABLE, CORNERS_TURN_TABLE}, symhash::SymHash, State}};

use super::stateset::StateSet;



pub struct StateSetFTO2Split {
    corners: usize,
    centers: usize
}

impl StateSet for StateSetFTO2Split {
    const NAME: &str = "fto2split";

    const SIZE: usize = StateCorners::SYM_SIZE * StateCenters::RAW_SIZE;

    const MAX_DEPTH: u8 = 10;

    fn solved() -> Self {
        Self {
            corners: StateCorners::solved().get_sym_hash(&CORNERS_SYMHASH_TABLE),
            centers: StateCenters::solved().get_hash()
        }
    }

    fn from_hash(hash: usize) -> Self {
        Self {
            corners: hash / StateCenters::RAW_SIZE,
            centers: hash % StateCenters::RAW_SIZE
        }
    }

    fn get_hash(&self) -> usize {
        self.corners * StateCenters::RAW_SIZE + self.centers
    }

    fn get_sym_hashes(&self) -> Vec<usize> {
        StateCorners::from_sym_hash(&CORNERS_SYMHASH_TABLE, self.corners).get_syms().iter().map(|&sym|
            self.corners * StateCenters::RAW_SIZE + StateCenters::apply_sym_table(&CENTERS_SYM_TABLE, self.centers, sym)
        ).collect()
    }

    fn get_next_state_sets(&self) -> Vec<Self> where Self: Sized {
        (0..StateCorners::NUM_TURNS).map(|turn| {
            let corners_comb_hash = StateCorners::apply_turn_table(&CORNERS_TURN_TABLE, self.corners, turn);
            let sym_hash = corners_comb_hash / StateCorners::NUM_SYMS as usize;
            let sym = (corners_comb_hash % StateCorners::NUM_SYMS as usize) as u8;
            let centers_raw_hash = StateCenters::apply_turn_table(&CENTERS_TURN_TABLE, self.centers, turn);
            let centers_hash = StateCenters::apply_sym_table(&CENTERS_SYM_TABLE, centers_raw_hash, sym);
            Self {
                corners: sym_hash,
                centers: centers_hash
            }
        }).collect()
    }
}

impl StateSetFTO2Split {
    pub fn from_parts(raw_cp_hash: usize, raw_co_hash: usize, raw_ce_hash: usize) -> Self {
        let (corners_sym_hash, sym) = StateCorners::from_hash(raw_cp_hash * 32 + raw_co_hash).get_sym_hash_sym(&CORNERS_SYMHASH_TABLE);
        let mut state_centers = StateCenters::from_hash(raw_ce_hash);
        state_centers.wrap_sym(sym);
        Self {
            corners: corners_sym_hash,
            centers: state_centers.get_hash()
        }
    }
}