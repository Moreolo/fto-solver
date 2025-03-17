pub mod statecp;
pub mod statecorners;
pub mod statecenters;
pub mod statefto2;
pub mod symhash;

pub mod statefto;
pub mod statepyra;

pub trait State: Sized + Clone {
    const RAW_SIZE: usize;
    const NUM_TURNS: u8;
    const NUM_SYMS: u8;
    fn solved() -> Self;
    fn from_hash(hash: usize) -> Self;
    fn get_hash(&self) -> usize;
    fn apply_turn(&mut self, turn: u8);
    fn wrap_sym(&mut self, sym: u8);

    fn get_next_states(&self) -> Vec<Self> {
        (0..Self::NUM_TURNS).map(|turn| {
            let mut adj = self.clone();
            adj.apply_turn(turn);
            adj
        }).collect()
    }

    fn get_sym_states(&self) -> Vec<Self> {
        (0..Self::NUM_SYMS).map(|sym| {
            let mut adj = self.clone();
            adj.wrap_sym(sym);
            adj
        }).collect()
    }
}

pub fn hash_permutation(arr: &[u8]) -> usize {
    let mut acc: usize = 0;
    let mut factor: usize = 1;
    for index in 0..arr.len() {
        if index > 1 {
            if index != 2 {
                factor *= index;
            }
            let higher = arr[..index].iter().filter(|&&other| other > arr[index]).count();
            acc += higher * factor;
        }
    }
    acc
}

pub fn hash_orientation(arr: &[bool]) -> usize {
    arr[1..].iter().fold(0, |acc, &oriented| if oriented {acc * 2} else {acc * 2 + 1})
}

pub fn hash_centers(arr: &[u8]) -> usize {
    hash_centers_any(arr, 3, 4)
}

fn hash_centers_any(arr: &[u8], same: usize, colors: usize) -> usize {
    let mut hash: Vec<usize> = vec![0; colors-1];
    let mut left: Vec<usize> = vec![same; colors];
    for &piece in arr {
        left[piece as usize] -= 1;
        for color in 0..piece {
            if left[color as usize] != 0 {
                hash[color as usize] += biko(left[color as usize..].iter().sum(), left[color as usize] - 1);
            }
        }
    }
    hash.iter().enumerate().fold(0, |acc, (index, &val)| acc * biko((colors - index) * same, same) + val)
}

fn hash_center_turn(arr: &[u8]) -> usize {
    arr.iter().fold(0, |acc, &turn| acc * 3 + turn as usize)
}

fn biko(n: usize, k: usize) -> usize {
    product(n - k, n) / product(1, k)
}

fn product(start: usize, end: usize) -> usize {
    (start+1..end+1).fold(1, |acc, n| acc * n)
}

pub fn unwrap_sym(sym: u8) -> (bool, bool, bool, u8) {
    (
        (sym & 1) == 1,
        ((sym >> 1) & 1) == 1,
        ((sym >> 2) & 1) == 1,
        sym >> 3
    )
}

pub fn unwrap_big_sym(sym: u8) -> (bool, bool, bool, bool, u8) {
    (
        (sym & 1) == 1,
        ((sym >> 1) & 1) == 1,
        ((sym >> 2) & 1) == 1,
        ((sym >> 3) & 1) == 1,
        sym >> 4
    )
}

pub fn unwrap_xbig_sym(sym: u8) -> (bool, bool, bool, bool, bool, u8) {
    (
        (sym & 1) == 1,
        ((sym >> 1) & 1) == 1,
        ((sym >> 2) & 1) == 1,
        ((sym >> 3) & 1) == 1,
        ((sym >> 4) & 1) == 1,
        sym >> 5
    )
}

pub fn unwrap_turn(turn: u8) -> (u8, bool) {
    (
        turn >> 1,
        turn & 1 == 0
    )
}

// IDs

const ID_CP: [u8; 6] = [0, 1, 2, 3, 4, 5];
const ID_CO: [bool; 6] = [true; 6];
const ID_CE: [u8; 4] = [0, 1, 2, 3];
const ID_EC: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const ID_CT: [u8; 4] = [0; 4];

// FTO2 Turns

const TURN_D_CP: [u8; 6] = [0, 1, 2, 5, 3, 4];
const TURN_D_CO: [bool; 6] = ID_CO;
const TURN_D_CE: [u8; 4] = [0, 3, 1, 2];

const TURN_B_CP: [u8; 6] = [0, 5, 1, 3, 4, 2];
const TURN_B_CO: [bool; 6] = [true, false, false, true, true, true];
const TURN_B_CE: [u8; 4] = [2, 1, 3, 0];

const TURN_L_CP: [u8; 6] = [4, 0, 2, 3, 1, 5];
const TURN_L_CO: [bool; 6] = [false, false, true, true, true, true];
const TURN_L_CE: [u8; 4] = [1, 2, 0, 3];

const TURN_R_CP: [u8; 6] = [2, 1, 3, 0, 4, 5];
const TURN_R_CO: [bool; 6] = [false, true, false, true, true, true];
const TURN_R_CE: [u8; 4] = [3, 0, 2, 1];

// FTO Extension

const TURN_U_CP: [u8; 6] = [1, 2, 0, 3, 4, 5];
const TURN_U_CO: [bool; 6] = ID_CO;

const TURN_F_CP: [u8; 6] = [3, 1, 2, 4, 0, 5];
const TURN_F_CO: [bool; 6] = [false, true, true, false, true, true];

const TURN_BL_CP: [u8; 6] = [0, 4, 2, 3, 5, 1];
const TURN_BL_CO: [bool; 6] = [true, false, true, true, false, true];

const TURN_BR_CP: [u8; 6] = [0, 1, 5, 2, 4, 3];
const TURN_BR_CO: [bool; 6] = [true, true, false, true, true, false];

const TURN_D_EP: [u8; 12] = [0, 1, 2, 3, 10, 5, 6, 4, 8, 9, 7, 11];
const TURN_B_EP: [u8; 12] = [0, 8, 2, 3, 4, 5, 6, 7, 9, 1, 10, 11];
const TURN_L_EP: [u8; 12] = [5, 1, 2, 3, 4, 6, 0, 7, 8, 9, 10, 11];
const TURN_R_EP: [u8; 12] = [0, 1, 11, 2, 4, 5, 6, 7, 8, 9, 10, 3];
const TURN_U_EP: [u8; 12] = [1, 2, 0, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const TURN_F_EP: [u8; 12] = [0, 1, 2, 4, 5, 3, 6, 7, 8, 9, 10, 11];
const TURN_BL_EP: [u8; 12] = [0, 1, 2, 3, 4, 5, 7, 8, 6, 9, 10, 11];
const TURN_BR_EP: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 9];

const TURN_D_CE1: [u8; 12] = [0, 1, 2, 3, 10, 11, 6, 4, 5, 9, 7, 8];
const TURN_B_CE1: [u8; 12] = [0, 8, 6, 3, 4, 5, 10, 7, 9, 1, 2, 11];
const TURN_L_CE1: [u8; 12] = [5, 3, 2, 7, 4, 6, 0, 1, 8, 9, 10, 11];
const TURN_R_CE1: [u8; 12] = [9, 1, 11, 2, 0, 5, 6, 7, 8, 4, 10, 3];
const TURN_U_CE1: [u8; 12] = TURN_U_EP;
const TURN_F_CE1: [u8; 12] = TURN_F_EP;
const TURN_BL_CE1: [u8; 12] = TURN_BL_EP;
const TURN_BR_CE1: [u8; 12] = TURN_BR_EP;

const TURN_D_CE2: [u8; 12] = TURN_U_CE1;
const TURN_B_CE2: [u8; 12] = TURN_F_CE1;
const TURN_L_CE2: [u8; 12] = TURN_BL_CE1;
const TURN_R_CE2: [u8; 12] = TURN_BR_CE1;
const TURN_U_CE2: [u8; 12] = TURN_D_CE1;
const TURN_F_CE2: [u8; 12] = TURN_B_CE1;
const TURN_BL_CE2: [u8; 12] = TURN_L_CE1;
const TURN_BR_CE2: [u8; 12] = TURN_R_CE1;

const TURN_D_CT: [u8; 4] = [1, 0, 0, 0];
const TURN_B_CT: [u8; 4] = [0, 1, 0, 0];
const TURN_L_CT: [u8; 4] = [0, 0, 1, 0];
const TURN_R_CT: [u8; 4] = [0, 0, 0, 1];

// FTO2 Syms

const SYM_Y_CP: [u8; 6] = [1, 2, 0, 4, 5, 3];
const SYM_Y_CO: [bool; 6] = ID_CO;
const SYM_Y_CE: [u8; 4] = [0, 2, 3, 1];

const SYM_Y2_CP: [u8; 6] = [2, 0, 1, 5, 3, 4];
const SYM_Y2_CO: [bool; 6] = ID_CO;
const SYM_Y2_CE: [u8; 4] = [0, 3, 1, 2];

const SYM_Z2_CP: [u8; 6] = [0, 3, 4, 1, 2, 5];
const SYM_Z2_CO: [bool; 6] = [false, true, false, true, false, false];
const SYM_Z2_CE: [u8; 4] = [1, 0, 3, 2];

const SYM_ZX2_CP: [u8; 6] = [5, 3, 2, 1, 4, 0];
const SYM_ZX2_CO: [bool; 6] = [true, false, false, false, false, true];
const SYM_ZX2_CE: [u8; 4] = [3, 2, 1, 0];

const SYM_M_CP: [u8; 6] = [0, 2, 1, 4, 3, 5];
const SYM_M_CO: [bool; 6] = [true, true, true, false, false, false];
const SYM_M_CE: [u8; 4] = [0, 1, 3, 2];

const SYM_X2_CP: [u8; 6] = [5, 4, 3, 2, 1, 0];
const SYM_X2_CO: [bool; 6] = [true, true, true, false, false, false];
const SYM_X2_CE: [u8; 4] = ID_CE;

// FTO Extension

const SYM_Y_EP: [u8; 12] = [1, 2, 0, 6, 7, 8, 9, 10, 11, 3, 4, 5];
const SYM_Y2_EP: [u8; 12] = [2, 0, 1, 9, 10, 11, 3, 4, 5, 6, 7, 8];
const SYM_Z2_EP: [u8; 12] = [3, 4, 5, 0, 1, 2, 11, 9, 10, 7, 8, 6];
const SYM_ZX2_EP: [u8; 12] = [10, 11, 9, 8, 6, 7, 4, 5, 3, 2, 0, 1];
const SYM_M_EP: [u8; 12] = [2, 1, 0, 5, 4, 3, 11, 10, 9, 8, 7, 6];
const SYM_X2_EP: [u8; 12] = [7, 4, 10, 9, 1, 8, 6, 0, 5, 3, 2, 11];

const SYM_Y_CE1: [u8; 12] = SYM_Y_EP;
const SYM_Y2_CE1: [u8; 12] = SYM_Y2_EP;
const SYM_Z2_CE1: [u8; 12] = SYM_Z2_EP;
const SYM_ZX2_CE1: [u8; 12] = SYM_ZX2_EP;
const SYM_M_CE1: [u8; 12] = [0, 2, 1, 3, 5, 4, 9, 11, 10, 6, 8, 7];

const SYM_Y_CE2: [u8; 12] = SYM_Y2_CE1;
const SYM_Y2_CE2: [u8; 12] = SYM_Y_CE1;
const SYM_Z2_CE2: [u8; 12] = SYM_Z2_CE1;
const SYM_ZX2_CE2: [u8; 12] = [8, 6, 7, 10, 11, 9, 1, 2, 0, 5, 3, 4];
const SYM_M_CE2: [u8; 12] = SYM_M_CE1;

const SYM_Y_CT: [u8; 12] = [0, 1, 2, 9, 10, 11, 3, 4, 5, 6, 7, 8];
const SYM_Y2_CT: [u8; 12] = [0, 1, 2, 6, 7, 8, 9, 10, 11, 3, 4, 5];
const SYM_Z2_CT: [u8; 12] = [3, 4, 5, 0, 1, 2, 9, 10, 11, 6, 7, 8];
const SYM_ZX2_CT: [u8; 12] = [6, 7, 8, 9, 10, 11, 0, 1, 2, 3, 4, 5];
const SYM_M_CT: [u8; 12] = SYM_M_CE1;

const MAP_CP_U: [u8; 6] = [0, 1, 2, 4, 7, 10];
const MAP_CP_F: [u8; 6] = [3, 6, 9, 11, 5, 8];