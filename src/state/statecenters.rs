use std::sync::LazyLock;

use crate::moving::{sym::SymTable, turn::TurnTable};

use super::*;


pub static CENTERS_TURN_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateCenters::get_turn_table());
pub static CENTERS_SYM_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateCenters::get_sym_table());

#[derive(Clone, Debug)]
pub struct StateCenters {
    ce: [u8; 4]
}

impl State for StateCenters {
    const RAW_SIZE: usize = 12;
    const NUM_TURNS: u8 = 8;
    const NUM_SYMS: u8 = 24;

    fn solved() -> Self {
        Self {
            ce: [0, 1, 2, 3]
        }
    }

    fn from_hash(hash: usize) -> Self {
        let mut ce = [4; 4];
        let mut left = hash;
        let mut pieces = vec![3, 2, 1, 0];
        let mut factor = 3;
        let mut parity = 0;
        for i in 0..2 {
            let p = left / factor;
            left %= factor;
            factor /= 3 - i;
            parity += p;
            ce[3 - i] = pieces.remove(p);
        }
        (ce[0], ce[1]) = if parity & 1 == 0 {
            (pieces[1], pieces[0])
        } else {
            (pieces[0], pieces[1])
        };
        Self {
            ce
        }
    }

    fn get_hash(&self) -> usize {
        hash_permutation(&self.ce)
    }

    fn apply_turn(&mut self, turn: u8) {
        let (face, cw) = unwrap_turn(turn);
        match face {
            0 => {
                if cw {
                    self.apply(TURN_D_CE);
                } else {
                    self.apply_inv(TURN_D_CE);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CE);
                } else {
                    self.apply_inv(TURN_B_CE);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CE);
                } else {
                    self.apply_inv(TURN_L_CE);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CE);
                } else {
                    self.apply_inv(TURN_R_CE);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (mirror, zx2, z2, y) = unwrap_sym(sym);
        if mirror {
            self.wrap(SYM_M_CE);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CE);
        }
        if z2 {
            self.wrap(SYM_Z2_CE);
        }
        if y == 1 {
            self.wrap(SYM_Y_CE);
        } else if y == 2 {
            self.wrap(SYM_Y2_CE);
        }
    }
}

impl TurnTable for StateCenters {
    const TURNTABLE_NAME: &str = "centers";
    const NUM_HASHES: usize = Self::RAW_SIZE;

    fn from_turntable_index(hash: usize) -> Self {
        Self::from_hash(hash)
    }

    fn get_turntable_entry(&self) -> usize {
        self.get_hash()
    }
}

impl SymTable for StateCenters {
    const SYMTABLE_NAME: &str = "centers";
}

impl StateCenters {
    fn apply(&mut self, permutation: [u8; 4]) {
        self.ce = {
            let mut new_ce = [4; 4];
            for i in 0..4 {
                new_ce[permutation[i] as usize] = self.ce[i];
            }
            new_ce
        }
    }

    fn apply_inv(&mut self, permutation: [u8; 4]) {
        self.ce = [0, 1, 2, 3].map(|i| self.ce[permutation[i] as usize])
    }

    fn wrap(&mut self, permutation: [u8; 4]) {
        self.ce = {
            let mut new_ce = [4; 4];
            for i in 0..4 {
                new_ce[permutation[i] as usize] = permutation[self.ce[i] as usize];
            }
            new_ce
        }
    }
}