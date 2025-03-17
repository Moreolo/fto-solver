use super::{statepyra::StatePyra, *};


#[derive(Debug, Clone)]
pub struct StateFTO {
    pub(crate) cp: [u8; 6],
    pub(crate) co: [bool; 6],
    pub(crate) ep: [u8; 12],
    pub(crate) ce1: [u8; 12],
    pub(crate) ce2: [u8; 12]
}

impl State for StateFTO {
    const RAW_SIZE: usize = 1; // actual size: 11_520 * 369_600 * 369_600 * 239_500_800
    const NUM_TURNS: u8 = 16;
    const NUM_SYMS: u8 = 48;

    fn solved() -> Self {
        Self {
            cp: ID_CP,
            co: ID_CO,
            ep: ID_EC,
            ce1: [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3],
            ce2: [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3]
        }
    }

    fn from_hash(_hash: usize) -> Self {
        panic!("Can't hash entire FTO")
    }

    fn get_hash(&self) -> usize {
        panic!("Can't hash entire FTO")
    }

    fn apply_turn(&mut self, turn: u8) {
        let (face, cw) = unwrap_turn(turn);
        match face {
            0 => {
                if cw {
                    self.apply(TURN_D_CP, TURN_D_CO, TURN_D_EP, TURN_D_CE1, TURN_D_CE2);
                } else {
                    self.apply_inv(TURN_D_CP, TURN_D_CO, TURN_D_EP, TURN_D_CE1, TURN_D_CE2);
                }
            }
            1 => {
                if cw {
                    self.apply(TURN_B_CP, TURN_B_CO, TURN_B_EP, TURN_B_CE1, TURN_B_CE2);
                } else {
                    self.apply_inv(TURN_B_CP, TURN_B_CO, TURN_B_EP, TURN_B_CE1, TURN_B_CE2);
                }
            }
            2 => {
                if cw {
                    self.apply(TURN_L_CP, TURN_L_CO, TURN_L_EP, TURN_L_CE1, TURN_L_CE2);
                } else {
                    self.apply_inv(TURN_L_CP, TURN_L_CO, TURN_L_EP, TURN_L_CE1, TURN_L_CE2);
                }
            }
            3 => {
                if cw {
                    self.apply(TURN_R_CP, TURN_R_CO, TURN_R_EP, TURN_R_CE1, TURN_R_CE2);
                } else {
                    self.apply_inv(TURN_R_CP, TURN_R_CO, TURN_R_EP, TURN_R_CE1, TURN_R_CE2);
                }
            }
            4 => {
                if cw {
                    self.apply(TURN_U_CP, TURN_U_CO, TURN_U_EP, TURN_U_CE1, TURN_U_CE2);
                } else {
                    self.apply_inv(TURN_U_CP, TURN_U_CO, TURN_U_EP, TURN_U_CE1, TURN_U_CE2);
                }
            }
            5 => {
                if cw {
                    self.apply(TURN_F_CP, TURN_F_CO, TURN_F_EP, TURN_F_CE1, TURN_F_CE2);
                } else {
                    self.apply_inv(TURN_F_CP, TURN_F_CO, TURN_F_EP, TURN_F_CE1, TURN_F_CE2);
                }
            }
            6 => {
                if cw {
                    self.apply(TURN_BL_CP, TURN_BL_CO, TURN_BL_EP, TURN_BL_CE1, TURN_BL_CE2);
                } else {
                    self.apply_inv(TURN_BL_CP, TURN_BL_CO, TURN_BL_EP, TURN_BL_CE1, TURN_BL_CE2);
                }
            }
            7 => {
                if cw {
                    self.apply(TURN_BR_CP, TURN_BR_CO, TURN_BR_EP, TURN_BR_CE1, TURN_BR_CE2);
                } else {
                    self.apply_inv(TURN_BR_CP, TURN_BR_CO, TURN_BR_EP, TURN_BR_CE1, TURN_BR_CE2);
                }
            }
            _ => {}
        }
    }

    fn wrap_sym(&mut self, sym: u8) {
        let (x2, mirror, zx2, z2, y) = unwrap_big_sym(sym);
        if x2 {
            self.wrap_x2();
        }
        if mirror {
            self.wrap(SYM_M_CP, SYM_M_CO, SYM_M_EP, SYM_M_CE1, SYM_M_CE2);
        }
        if zx2 {
            self.wrap(SYM_ZX2_CP, SYM_ZX2_CO, SYM_ZX2_EP, SYM_ZX2_CE1, SYM_ZX2_CE2);
        }
        if z2 {
            self.wrap(SYM_Z2_CP, SYM_Z2_CO, SYM_Z2_EP, SYM_Z2_CE1, SYM_Z2_CE2);
        }
        if y == 1 {
            self.wrap(SYM_Y_CP, SYM_Y_CO, SYM_Y_EP, SYM_Y_CE1, SYM_Y_CE2);
        } else if y == 2 {
            self.wrap(SYM_Y2_CP, SYM_Y2_CO, SYM_Y2_EP, SYM_Y2_CE1, SYM_Y2_CE2);
        }
    }
}

impl StateFTO {
    pub fn from_pyra(pyra: &StatePyra) -> Self {
        let mut fto = Self::solved();
        let (cp, co, ct) = pyra.get_parts();
        match ct[0] {
            1 => fto.apply(ID_CP, ID_CO, TURN_D_EP, ID_EC, ID_EC),
            2 => fto.apply_inv(ID_CP, ID_CO, TURN_D_EP, ID_EC, ID_EC),
            _ => {}
        }
        match ct[1] {
            1 => fto.apply(ID_CP, ID_CO, TURN_B_EP, ID_EC, ID_EC),
            2 => fto.apply_inv(ID_CP, ID_CO, TURN_B_EP, ID_EC, ID_EC),
            _ => {}
        }
        match ct[2] {
            1 => fto.apply(ID_CP, ID_CO, TURN_L_EP, ID_EC, ID_EC),
            2 => fto.apply_inv(ID_CP, ID_CO, TURN_L_EP, ID_EC, ID_EC),
            _ => {}
        }
        match ct[3] {
            1 => fto.apply(ID_CP, ID_CO, TURN_R_EP, ID_EC, ID_EC),
            2 => fto.apply_inv(ID_CP, ID_CO, TURN_R_EP, ID_EC, ID_EC),
            _ => {}
        }
        fto.apply_permutation_ce1(cp, co);
        fto.cp = cp;
        fto.co = co;
        fto
    }

    pub fn apply(&mut self, cp: [u8; 6], co: [bool; 6], ep: [u8; 12], ce1: [u8; 12], ce2: [u8; 12]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = self.cp[i];
                new_co[cp[i] as usize] = self.co[i] == co[i];
            }
            (new_cp, new_co)
        };
        (self.ep, self.ce1, self.ce2) = {
            let mut new_ep = [12; 12];
            let mut new_ce1 = [12; 12];
            let mut new_ce2 = [12; 12];
            for i in 0..12 {
                new_ep[ep[i] as usize] = self.ep[i];
                new_ce1[ce1[i] as usize] = self.ce1[i];
                new_ce2[ce2[i] as usize] = self.ce2[i];
            }
            (new_ep, new_ce1, new_ce2)
        };
    }

    fn apply_inv(&mut self, cp: [u8; 6], co: [bool; 6], ep: [u8; 12], ce1: [u8; 12], ce2: [u8; 12]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[i as usize] = self.cp[cp[i] as usize];
                new_co[i as usize] = self.co[cp[i] as usize] == co[i];
            }
            (new_cp, new_co)
        };
        self.ep = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|i| self.ep[ep[i] as usize]);
        self.ce1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|i| self.ce1[ce1[i] as usize]);
        self.ce2 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(|i| self.ce2[ce2[i] as usize]);
    }

    fn wrap(&mut self, cp: [u8; 6], co: [bool; 6], ep: [u8; 12], ce1: [u8; 12], ce2: [u8; 12]) {
        (self.cp, self.co) = {
            let mut new_cp = [6; 6];
            let mut new_co = [true; 6];
            for i in 0..6 {
                new_cp[cp[i] as usize] = cp[self.cp[i] as usize];
                new_co[cp[i] as usize] = (self.co[i] == co[i]) == co[self.cp[i] as usize];
            }
            (new_cp, new_co)
        };
        (self.ep, self.ce1, self.ce2) = {
            let mut new_ep = [12; 12];
            let mut new_ce1 = [12; 12];
            let mut new_ce2 = [12; 12];
            for i in 0..12 {
                new_ep[ep[i] as usize] = ep[self.ep[i] as usize];
                new_ce1[ce1[i] as usize] = ce1[self.ce1[i] as usize * 3] / 3;
                new_ce2[ce2[i] as usize] = ce2[self.ce2[i] as usize * 3] / 3;
            }
            (new_ep, new_ce1, new_ce2)
        };
    }

    fn wrap_x2(&mut self) {
        self.wrap(SYM_X2_CP, SYM_X2_CO, SYM_X2_EP, ID_EC, ID_EC);
        (self.ce1, self.ce2) = (self.ce2, self.ce1);
    }

    fn apply_permutation_ce1(&mut self, cp: [u8; 6], co: [bool; 6]) {
        self.ce1 = {
            let mut new_ce1 = [12; 12];
            for i in 0..6 {
                let (new_up, new_down) = if co[i] {
                    (MAP_CP_U[cp[i] as usize], MAP_CP_F[cp[i] as usize])
                } else {
                    (MAP_CP_F[cp[i] as usize], MAP_CP_U[cp[i] as usize])
                };
                new_ce1[MAP_CP_U[i] as usize] = self.ce1[new_up as usize];
                new_ce1[MAP_CP_F[i] as usize] = self.ce1[new_down as usize];
            }
            new_ce1
        };
        
    }
}