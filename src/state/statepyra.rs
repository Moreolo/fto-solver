use std::sync::LazyLock;

use crate::moving::turn::TurnTable;

use super::{statefto::StateFTO, symhash::SymHash, *};

pub static PYRA_SYMHASH_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StatePyra::get_symhash_table());
pub static PYRA_TURN_TABLE: LazyLock<Vec<usize>> = LazyLock::new(|| StatePyra::get_turn_table());

#[derive(Clone)]
pub struct StatePyra {
    cp: [u8; 6],
    co: [bool; 6],
    ct: [u8; 4],
}

impl State for StatePyra {
    const RAW_SIZE: usize = 360 * 32 * 81;

    const NUM_TURNS: u8 = 8 * 2;

    const NUM_SYMS: u8 = 24 * 2;

    fn solved() -> Self {
        Self {
            cp: ID_CP,
            co: ID_CO,
            ct: ID_CT
        }
    }

    fn from_hash(hash: usize) -> Self {
        let mut cp = [6; 6];
        let mut left = (hash / 81) >> 5;
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
        left = (hash / 81) & 31;
        let mut even = true;
        for i in 0..5 {
            let o = left & 1 == 0;
            left /= 2;
            co[5 - i] = o;
            even = even == o;
        }
        co[0] = even;

        let mut ct = ID_CT;
        let mut left = (hash % 81) as u8;
        for i in 0..4 {
            ct[3 - i] = left % 3;
            left /= 3;
        }

        Self {
            cp,
            co,
            ct
        }
    }

    fn get_hash(&self) -> usize {
        (hash_permutation(&self.cp) * 32 + hash_orientation(&self.co)) * 81 + hash_center_turn(&self.ct)
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
                    self.apply(TURN_D_CP, TURN_D_CO, TURN_D_CT);
                } else {
                    self.apply_inv(TURN_D_CP, TURN_D_CO, TURN_D_CT);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CP, TURN_B_CO, TURN_B_CT);
                } else {
                    self.apply_inv(TURN_B_CP, TURN_B_CO, TURN_B_CT);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CP, TURN_L_CO, TURN_L_CT);
                } else {
                    self.apply_inv(TURN_L_CP, TURN_L_CO, TURN_L_CT);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CP, TURN_R_CO, TURN_R_CT);
                } else {
                    self.apply_inv(TURN_R_CP, TURN_R_CO, TURN_R_CT);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (inv, mirror, zx2, z2, y) = unwrap_big_sym(sym);
        if inv {
            self.inverse();
        }
        if mirror {
            self.wrap(SYM_M_CP, SYM_M_CO, SYM_M_CT);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CP, SYM_ZX2_CO, SYM_ZX2_CT);
        }
        if z2 {
            self.wrap(SYM_Z2_CP, SYM_Z2_CO, SYM_Z2_CT);
        }
        if y == 1 {
            self.wrap(SYM_Y_CP, SYM_Y_CO, SYM_Y_CT);
        } else if y == 2 {
            self.wrap(SYM_Y2_CP, SYM_Y2_CO, SYM_Y2_CT);
        }
    }
}

impl SymHash for StatePyra {
    const SYM_NAME: &str = "pyra";
    const SYM_SIZE: usize = 21073;
}

impl TurnTable for StatePyra {
    const TURNTABLE_NAME: &str = "pyra";
    const NUM_HASHES: usize = Self::SYM_SIZE;

    fn from_turntable_index(hash: usize) -> Self {
        Self::from_sym_hash(&PYRA_SYMHASH_TABLE, hash)
    }

    fn get_turntable_entry(&self) -> usize {
        let (sym_hash, sym) = self.get_sym_hash_sym(&PYRA_SYMHASH_TABLE);
        sym_hash * Self::NUM_SYMS as usize + sym as usize
    }
}

impl StatePyra {
    pub fn scrambled() -> Self {
        Self::from_hash(rand::random_range(0..Self::RAW_SIZE))
    }

    pub fn get_parts(&self) -> ([u8; 6], [bool; 6], [u8; 4]) {
        (self.cp, self.co, self.ct)
    }

    pub fn from_fto(fto: &StateFTO) -> Self {
        let ct = [
            match fto.ep[4] {
                4 => 0,
                7 => 1,
                10 => 2,
                _ => panic!("FTO not in Pyra")
            },
            match fto.ep[1] {
                1 => 0,
                9 => 1,
                8 => 2,
                _ => panic!("FTO not in Pyra")
            },
            match fto.ep[0] {
                0 => 0,
                6 => 1,
                5 => 2,
                _ => panic!("FTO not in Pyra")
            },
            match fto.ep[2] {
                2 => 0,
                3 => 1,
                11 => 2,
                _ => panic!("FTO not in Pyra")
            }
        ];
        Self {
            cp: fto.cp,
            co: fto.co,
            ct
        }

    }

    fn apply(&mut self, cp: [u8; 6], co: [bool; 6], ct: [u8; 4]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = self.cp[i];
                new_co[cp[i] as usize] = self.co[i] == co[i];
            }
            (new_cp, new_co)
        };
        for i in 0..4 {
            self.ct[i] = (self.ct[i] + ct[i]) % 3;
        };
    }

    fn apply_inv(&mut self, cp: [u8; 6], co: [bool; 6], ct: [u8; 4]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[i as usize] = self.cp[cp[i] as usize];
                new_co[i as usize] = self.co[cp[i] as usize] == co[i];
            }
            (new_cp, new_co)
        };
        for i in 0..4 {
            self.ct[i] = (3 + self.ct[i] - ct[i]) % 3;
        };
    }

    fn wrap(&mut self, cp: [u8; 6], co: [bool; 6], ct: [u8; 12]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = cp[self.cp[i] as usize];
                new_co[cp[i] as usize] = (self.co[i] == co[i]) == co[self.cp[i] as usize];
            }
            (new_cp, new_co)
        };
        self.ct = {
            let mut new_ct = [3; 4];
            for i in 0..4 {
                let new_raw_c = ct[self.ct[i] as usize + i * 3];
                new_ct[new_raw_c as usize / 3] = new_raw_c % 3;
            }
            new_ct
        };
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
        for i in 0..4 {
            self.ct[i] = match self.ct[i] {
                1 => 2,
                2 => 1,
                _ => 0
            };
        }
    }
}