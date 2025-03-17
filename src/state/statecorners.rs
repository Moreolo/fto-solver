use std::sync::LazyLock;

use crate::moving::turn::TurnTable;

use super::{symhash::SymHash, *};

pub static CORNERS_SYMHASH_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateCorners::get_symhash_table());
pub static CORNERS_TURN_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateCorners::get_turn_table());

#[derive(Clone, Debug)]
pub struct StateCorners {
    cp: [u8; 6],
    co: [bool; 6]
}

impl State for StateCorners {
    const RAW_SIZE: usize = 11_520;
    const NUM_TURNS: u8 = 8;
    const NUM_SYMS: u8 = 24;

    fn solved() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5],
            co: [true; 6]
        }
    }

    fn from_hash(hash: usize) -> Self {
        let mut cp = [6; 6];
        let mut left = hash >> 5;
        let mut pieces = vec![5, 4, 3, 2, 1, 0];
        let mut factor = 60;
        let mut parity = 0;
        for i in 0..4 {
            let p = left / factor;
            left %= factor;
            factor /= 5 - i;
            parity += p;
            cp[5 - i] = pieces.remove(p);
        }
        (cp[0], cp[1]) = if parity & 1 == 0 {
            (pieces[1], pieces[0])
        } else {
            (pieces[0], pieces[1])
        };

        let mut co = [true; 6];
        left = hash & 31;
        let mut even = true;
        for i in 0..5 {
            let o = left & 1 == 0;
            left /= 2;
            co[5 - i] = o;
            even = even == o;
        }
        co[0] = even;
        Self {
            cp,
            co
        }
    }

    fn get_hash(&self) -> usize {
        hash_permutation(&self.cp) * 32 + hash_orientation(&self.co)
    }

    fn apply_turn(&mut self, turn: u8) {
        let (face, cw) = unwrap_turn(turn);
        match face {
            0 => {
                if cw {
                    self.apply(TURN_D_CP, TURN_D_CO);
                } else {
                    self.apply_inv(TURN_D_CP, TURN_D_CO);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CP, TURN_B_CO);
                } else {
                    self.apply_inv(TURN_B_CP, TURN_B_CO);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CP, TURN_L_CO);
                } else {
                    self.apply_inv(TURN_L_CP, TURN_L_CO);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CP, TURN_R_CO);
                } else {
                    self.apply_inv(TURN_R_CP, TURN_R_CO);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (mirror, zx2, z2, y) = unwrap_sym(sym);
        if mirror {
            self.wrap(SYM_M_CP, SYM_M_CO);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CP, SYM_ZX2_CO);
        }
        if z2 {
            self.wrap(SYM_Z2_CP, SYM_Z2_CO);
        }
        if y == 1 {
            self.wrap(SYM_Y_CP, SYM_Y_CO);
        } else if y == 2 {
            self.wrap(SYM_Y2_CP, SYM_Y2_CO);
        }
    }
}

impl SymHash for StateCorners {
    const SYM_NAME: &str = "corners";
    const SYM_SIZE: usize = 504;
}

impl TurnTable for StateCorners {
    const TURNTABLE_NAME: &str = "corners";
    const NUM_HASHES: usize = Self::SYM_SIZE;

    fn from_turntable_index(hash: usize) -> Self {
        Self::from_sym_hash(&CORNERS_SYMHASH_TABLE, hash)
    }

    fn get_turntable_entry(&self) -> usize {
        let (sym_hash, sym) = self.get_sym_hash_sym(&CORNERS_SYMHASH_TABLE);
        sym_hash * Self::NUM_SYMS as usize + sym as usize
    }
}

impl StateCorners {
    fn apply(&mut self, permutation: [u8; 6], orientation: [bool; 6]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[permutation[i] as usize] = self.cp[i];
                new_co[permutation[i] as usize] = self.co[i] == orientation[i];
            }
            (new_cp, new_co)
        }
    }

    fn apply_inv(&mut self, permutation: [u8; 6], orientation: [bool; 6]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[i] = self.cp[permutation[i] as usize];
                new_co[i] = self.co[permutation[i] as usize] == orientation[i];
            }
            (new_cp, new_co)
        }
    }

    fn wrap(&mut self, permutation: [u8; 6], orientation: [bool; 6]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[permutation[i] as usize] = permutation[self.cp[i] as usize];
                new_co[permutation[i] as usize] = (self.co[i] == orientation[i]) == orientation[self.cp[i] as usize];
            }
            (new_cp, new_co)
        }
    }
}