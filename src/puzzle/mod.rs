use std::fmt::Display;

use colored::{ColoredString, Colorize};

pub mod fto2;
pub mod fto;

pub trait Puzzle: Display {
    fn do_notation(&mut self, notation: &str);

    fn get_notation(turn: u8) -> &'static str;

    fn do_sequence(&mut self, sequence: &str) {
        let turns: Vec<&str> = sequence.split(' ').collect();
        turns.iter().for_each(|notation| self.do_notation(notation));
    }

    fn get_sequence_len(sequence: &str) -> usize {
        let turns: Vec<&str> = sequence.split(' ').collect();
        turns.iter().count()
    }

    fn get_sequence_inv(sequence: &str) -> String {
        let mut turns: Vec<&str> = sequence.split(' ').collect();
        turns.reverse();
        let mut new_sequence = String::new();
        for turn in turns {
            new_sequence += turn;
            if turn.ends_with("'") {
                new_sequence.pop();
            } else {
                new_sequence.push('\'');
            }
            new_sequence.push(' ');
        }
        new_sequence.pop();
        new_sequence
    }

    fn color_center_sticker(sticker: &str, center: u8) -> ColoredString {
        match center {
            0 => sticker.white(),
            1 => sticker.green(),
            2 => sticker.truecolor(255, 128, 0),
            3 => sticker.truecolor(100, 100, 100),
            4 => sticker.truecolor(255, 255, 0),
            5 => sticker.blue(),
            6 => sticker.purple(),
            7 => sticker.red(),
            _ => sticker.bright_magenta()
        }
    }

    fn color_corner_sticker(sticker: &str, corner: u8, oriented: bool, number: u8) -> ColoredString {
        match corner {
            0 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.white()
                    } else {
                        sticker.green()
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.red()
                    } else {
                        sticker.purple()
                    }
                }
            }
            1 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.white()
                    } else {
                        sticker.truecolor(255, 128, 0)
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.purple()
                    } else {
                        sticker.blue()
                    }
                }
            }
            2 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.white()
                    } else {
                        sticker.truecolor(100, 100, 100)
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.blue()
                    } else {
                        sticker.red()
                    }
                }
            }
            3 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.green()
                    } else {
                        sticker.truecolor(100, 100, 100)
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.red()
                    } else {
                        sticker.truecolor(255, 255, 0)
                    }
                }
            }
            4 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.truecolor(255, 128, 0)
                    } else {
                        sticker.green()
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.purple()
                    } else {
                        sticker.truecolor(255, 255, 0)
                    }
                }
            }
            5 => {
                if number & 1 == 0 {
                    if (number == 0) == oriented {
                        sticker.truecolor(100, 100, 100)
                    } else {
                        sticker.truecolor(255, 128, 0)
                    }
                } else {
                    if (number == 1) == oriented {
                        sticker.blue()
                    } else {
                        sticker.truecolor(255, 255, 0)
                    }
                }
            }
            _ => sticker.bright_magenta()
        }
    }

    fn color_edge_sticker(sticker: &str, edge: u8, u_orbit: bool) -> ColoredString {
        match edge {
            0 => {
                if u_orbit {
                    sticker.white()
                } else {
                    sticker.purple()
                }
            }
            1 => {
                if u_orbit {
                    sticker.white()
                } else {
                    sticker.blue()
                }
            }
            2 => {
                if u_orbit {
                    sticker.white()
                } else {
                    sticker.red()
                }
            }
            3 => {
                if u_orbit {
                    sticker.green()
                } else {
                    sticker.red()
                }
            }
            4 => {
                if u_orbit {
                    sticker.green()
                } else {
                    sticker.truecolor(255, 255, 0)
                }
            }
            5 => {
                if u_orbit {
                    sticker.green()
                } else {
                    sticker.purple()
                }
            }
            6 => {
                if u_orbit {
                    sticker.truecolor(255, 128, 0)
                } else {
                    sticker.purple()
                }
            }
            7 => {
                if u_orbit {
                    sticker.truecolor(255, 128, 0)
                } else {
                    sticker.yellow()
                }
            }
            8 => {
                if u_orbit {
                    sticker.truecolor(255, 128, 0)
                } else {
                    sticker.blue()
                }
            }
            9 => {
                if u_orbit {
                    sticker.truecolor(100, 100, 100)
                } else {
                    sticker.blue()
                }
            }
            10 => {
                if u_orbit {
                    sticker.truecolor(100, 100, 100)
                } else {
                    sticker.truecolor(255, 255, 0)
                }
            }
            11 => {
                if u_orbit {
                    sticker.truecolor(100, 100, 100)
                } else {
                    sticker.red()
                }
            }
            _ => sticker.bright_magenta()
        }
    }
}

// gray sticker.truecolor(100, 100, 100)
// orange sticker.truecolor(255, 128, 0)
// yellow sticker.truecolor(255, 255, 0)