use fto_solver::{solver::solverpyra::SolverPyra, state::{statefto::StateFTO, statepyra::StatePyra}};

fn main() {
    // let mut fto = StateFTO2::scrambled();
    // println!("{fto}");
    // // fto.inverse();
    // let solver = SolverFTO2::new();
    // let scramble = solver.solve(&fto);
    // println!("{scramble}");
    // fto.do_sequence(&scramble);
    // println!("{fto}");

    // println!("{solution} ({})", StateFTO2::get_sequence_len(solution.as_str()));
    // fto.do_sequence(&solution);
    // println!("{fto}");

    let mut pyra = StatePyra::scrambled();
    let fto = StateFTO::from_pyra(&pyra);
    println!("{fto}");
    pyra.inverse();
    let solver = SolverPyra::new();
    let scramble = solver.solve(&pyra);
    println!("{scramble}");

    // // let poss: u128 = 43_252_003_274_489_856_000;
    // let poss: u128 = 31_408_133_379_194_880_000_000;
    // let comb: u128 = _calc_comb_fto(21, 0);
    // println!("{}", comb);
    // if comb > poss {
    //     println!("Exhausted Permutations");
    // }
}

fn _calc_comb_std(depth: u8, split: u8) -> u128 {
    if depth == 0 {
        2
    } else {
        match split {
            0 => 9 * _calc_comb_std(depth-1, 1),
            1 => 12 * _calc_comb_std(depth-1, 1) + 3 * _calc_comb_std(depth-1, 2) / 2,
            2 => 12 * _calc_comb_std(depth-1, 1),
            _ => 1
        }
    }
}

fn _calc_comb_fto(depth: u8, split: u8) -> u128 {
    if depth == 0 {
        1
    } else {
        match split {
            0 => 16 * _calc_comb_fto(depth-1, 1),
            1 => 12 * _calc_comb_fto(depth-1, 1) + _calc_comb_fto(depth-1, 2),
            2 => 12 * _calc_comb_fto(depth-1, 1),
            _ => 1
        }
    }
}