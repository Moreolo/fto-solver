use std::sync::LazyLock;

use crate::moving::turn::TurnTable;

use super::{symhash::SymHash, *};

pub static FTO2_SYMHASH_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateFTO2::get_symhash_table());
pub static FTO2_TURN_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StateFTO2::get_turn_table());

#[derive(Clone, Debug)]
pub struct StateFTO2 {
    pub(crate) cp: [u8; 6],
    pub(crate) co: [bool; 6],
    pub(crate) ce: [u8; 4]
}

impl State for StateFTO2 {
    const RAW_SIZE: usize = 360 * 32 * 12;
    const NUM_TURNS: u8 = 8 * 2;
    const NUM_SYMS: u8 = 48 * 2;

    fn solved() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5],
            co: [true; 6],
            ce: [0, 1, 2, 3]
        }
    }

    fn from_hash(hash: usize) -> Self {
        let mut cp = [6; 6];
        let mut left = (hash / 12) >> 5;
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
        left = (hash / 12) & 31;
        let mut even = true;
        for i in 0..5 {
            let o = left & 1 == 0;
            left /= 2;
            co[5 - i] = o;
            even = even == o;
        }
        co[0] = even;

        let mut ce = [4; 4];
        left = hash % 12;
        pieces = vec![3, 2, 1, 0];
        factor = 3;
        parity = 0;
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
            cp,
            co,
            ce
        }
    }

    fn get_hash(&self) -> usize {
        (hash_permutation(&self.cp) * 32 + hash_orientation(&self.co)) * 12 + hash_permutation(&self.ce)
    }

    fn apply_turn(&mut self, turn: u8) {
        let (mut face, cw) = unwrap_turn(turn);
        if face >= 4 {
            self.inverse();
            face -= 4;
        }
        match face {
            0 => {
                if cw {
                    self.apply(TURN_D_CP, TURN_D_CO, TURN_D_CE);
                } else {
                    self.apply_inv(TURN_D_CP, TURN_D_CO, TURN_D_CE);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CP, TURN_B_CO, TURN_B_CE);
                } else {
                    self.apply_inv(TURN_B_CP, TURN_B_CO, TURN_B_CE);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CP, TURN_L_CO, TURN_L_CE);
                } else {
                    self.apply_inv(TURN_L_CP, TURN_L_CO, TURN_L_CE);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CP, TURN_R_CO, TURN_R_CE);
                } else {
                    self.apply_inv(TURN_R_CP, TURN_R_CO, TURN_R_CE);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (inv, x2, mirror, zx2, z2, y) = unwrap_xbig_sym(sym);
        if inv {
            self.inverse();
        }
        if x2 {
            self.wrap_x2();
        }
        if mirror {
            self.wrap(SYM_M_CP, SYM_M_CO, SYM_M_CE);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CP, SYM_ZX2_CO, SYM_ZX2_CE);
        }
        if z2 {
            self.wrap(SYM_Z2_CP, SYM_Z2_CO, SYM_Z2_CE);
        }
        if y == 1 {
            self.wrap(SYM_Y_CP, SYM_Y_CO, SYM_Y_CE);
        } else if y == 2 {
            self.wrap(SYM_Y2_CP, SYM_Y2_CO, SYM_Y2_CE);
        }
    }
}

impl SymHash for StateFTO2 {
    const SYM_NAME: &str = "fto2";
    const SYM_SIZE: usize = 1_815;
}

impl TurnTable for StateFTO2 {
    const TURNTABLE_NAME: &str = "fto2";
    const NUM_HASHES: usize = Self::SYM_SIZE;

    fn from_turntable_index(hash: usize) -> Self {
        Self::from_sym_hash(&FTO2_SYMHASH_TABLE, hash)
    }

    fn get_turntable_entry(&self) -> usize {
        let (sym_hash, sym) = self.get_sym_hash_sym(&FTO2_SYMHASH_TABLE);
        sym_hash * Self::NUM_SYMS as usize + sym as usize
    }
}

impl StateFTO2 {
    pub fn scrambled() -> Self {
        Self::from_hash(rand::random_range(0..Self::RAW_SIZE))
    }

    pub fn get_parts(&self) -> (usize, usize, usize) {
        (hash_permutation(&self.cp), hash_orientation(&self.co), hash_permutation(&self.ce))
    }

    fn apply(&mut self, cp: [u8; 6], co: [bool; 6], ce: [u8; 4]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = self.cp[i];
                new_co[cp[i] as usize] = self.co[i] == co[i];
            }
            (new_cp, new_co)
        };
        self.ce = {
            let mut new_ce = [4; 4];
            for i in 0..4 {
                new_ce[ce[i] as usize] = self.ce[i];
            }
            new_ce
        }
    }

    fn apply_inv(&mut self, cp: [u8; 6], co: [bool; 6], ce: [u8; 4]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[i as usize] = self.cp[cp[i] as usize];
                new_co[i as usize] = self.co[cp[i] as usize] == co[i];
            }
            (new_cp, new_co)
        };
        self.ce = [0, 1, 2, 3].map(|i| self.ce[ce[i] as usize])
    }

    fn wrap(&mut self, cp: [u8; 6], co: [bool; 6], ce: [u8; 4]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = cp[self.cp[i] as usize];
                new_co[cp[i] as usize] = (self.co[i] == co[i]) == co[self.cp[i] as usize];
            }
            (new_cp, new_co)
        };
        self.ce = {
            let mut new_ce = [4; 4];
            for i in 0..4 {
                new_ce[ce[i] as usize] = ce[self.ce[i] as usize];
            }
            new_ce
        }
    }

    fn wrap_x2(&mut self) {
        let mut x2_ce = [0, 1, 2, 3];
        if self.ce[0] != 0 && self.ce[1] != 0 {
            self.apply(SYM_ZX2_CP, SYM_ZX2_CO, SYM_ZX2_CE);
            x2_ce = Self::apply_oocenters(x2_ce, SYM_ZX2_CE);
            x2_ce = Self::apply_oocenters(x2_ce, SYM_Z2_CE);
        }
        if self.ce[0] != 0 {
            self.apply(SYM_Z2_CP, SYM_Z2_CO, SYM_Z2_CE);
            x2_ce = Self::apply_oocenters(x2_ce, SYM_Z2_CE);
        }
        if self.ce[1] == 2 {
            self.apply(SYM_Y_CP, SYM_Y_CO, SYM_Y_CE);
            x2_ce = Self::apply_oocenters(x2_ce, SYM_Y_CE);
        } else if self.ce[1] == 3 {
            self.apply(SYM_Y2_CP, SYM_Y2_CO, SYM_Y2_CE);
            x2_ce = Self::apply_oocenters(x2_ce, SYM_Y2_CE);
        }
        self.wrap(SYM_X2_CP, SYM_X2_CO, SYM_X2_CE);
        self.ce = x2_ce;
    }

    fn apply_oocenters(arr: [u8; 4], ce: [u8; 4]) -> [u8; 4] {
        [0, 1, 2, 3].map(|i| arr[ce[i] as usize])
    }

    pub fn inverse(&mut self) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[self.cp[i] as usize] = i as u8;
                new_co[self.cp[i] as usize] = self.co[i];
            }
            (new_cp, new_co)
        };
        self.ce = {
            let mut new_ce = [4; 4];
            for i in 0..4 {
                new_ce[self.ce[i] as usize] = i as u8;
            }
            new_ce
        }
    }
}