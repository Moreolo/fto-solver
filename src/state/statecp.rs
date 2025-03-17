use super::{symhash::SymHash, *};

#[derive(Clone, Debug)]
pub struct StateCP {
    cp: [u8; 6]
}

impl State for StateCP {
    const RAW_SIZE: usize = 360;
    const NUM_TURNS: u8 = 8;
    const NUM_SYMS: u8 = 24;

    fn solved() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5]
        }
    }

    fn from_hash(hash: usize) -> Self {
        let mut cp = [6; 6];
        let mut left = hash;
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
        Self {
            cp
        }
    }

    fn get_hash(&self) -> usize {
        hash_permutation(&self.cp)
    }

    fn apply_turn(&mut self, turn: u8) {
        let (face, cw) = unwrap_turn(turn);
        match face {
            0 => {
                if cw {
                    self.apply(TURN_D_CP);
                } else {
                    self.apply_inv(TURN_D_CP);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CP);
                } else {
                    self.apply_inv(TURN_B_CP);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CP);
                } else {
                    self.apply_inv(TURN_L_CP);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CP);
                } else {
                    self.apply_inv(TURN_R_CP);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (mirror, zx2, z2, y) = unwrap_sym(sym);
        if mirror {
            self.wrap(SYM_M_CP);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CP);
        }
        if z2 {
            self.wrap(SYM_Z2_CP);
        }
        if y == 1 {
            self.wrap(SYM_Y_CP);
        } else if y == 2 {
            self.wrap(SYM_Y2_CP);
        }
    }
}

impl SymHash for StateCP {
    const SYM_NAME: &str = "cp";
    const SYM_SIZE: usize = 22;
}

impl StateCP {
    fn apply(&mut self, permutation: [u8; 6]) {
        self.cp = {
            let mut new_cp = [6; 6];
            for i in 0..6 {
                new_cp[permutation[i] as usize] = self.cp[i];
            }
            new_cp
        }
    }

    fn apply_inv(&mut self, permutation: [u8; 6]) {
        self.cp = [0, 1, 2, 3, 4, 5].map(|i| self.cp[permutation[i] as usize])
    }

    fn wrap(&mut self, permutation: [u8; 6]) {
        self.cp = {
            let mut new_cp = [6; 6];
            for i in 0..6 {
                new_cp[permutation[i] as usize] = permutation[self.cp[i] as usize];
            }
            new_cp
        }
    }
}