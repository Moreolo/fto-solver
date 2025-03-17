use std::fmt;

use colored::Colorize;

use crate::state::{statefto2::StateFTO2, unwrap_turn, State};

use super::Puzzle;


impl Puzzle for StateFTO2 {
    fn do_notation(&mut self, _notation: &str) {
        match _notation {
            "D" => self.apply_turn(0),
            "D'" => self.apply_turn(1),
            "B" => self.apply_turn(2),
            "B'" => self.apply_turn(3),
            "L" => self.apply_turn(4),
            "L'" => self.apply_turn(5),
            "R" => self.apply_turn(6),
            "R'" => self.apply_turn(7),
            _ => println!("Didn't execute turn")
        }
    }

    fn get_notation(turn: u8) -> &'static str {
        let (face, cw) = unwrap_turn(turn);
        match face {
            0 => if cw {"D"} else {"D'"},
            1 => if cw {"B"} else {"B'"},
            2 => if cw {"L"} else {"L'"},
            3 => if cw {"R"} else {"R'"},
            _ => ""
        }
    }
}

impl fmt::Display for StateFTO2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} {} /{} |{} \\{} {}{}\n{} /{} {}{} |{}{} {} \\{}\n-----------------\n{} \\{} {}{} |{}{} {} /{}\n{}{} {} \\{} |{} /{} {}{}",
            Self::color_corner_sticker("◤", self.cp[5], self.co[5], 1),
            "◢".blue(),
            Self::color_corner_sticker("◤", self.cp[2], self.co[2], 1),
            Self::color_corner_sticker("◢", self.cp[2], self.co[2], 0),
            Self::color_corner_sticker("◣", self.cp[2], self.co[2], 3),
            Self::color_corner_sticker("◥", self.cp[2], self.co[2], 2),
            Self::color_center_sticker("◣", self.ce[3]),
            Self::color_corner_sticker("◥", self.cp[5], self.co[5], 0),

            Self::color_corner_sticker("◤", self.cp[1], self.co[1], 3),
            Self::color_corner_sticker("◢", self.cp[1], self.co[1], 0),
            Self::color_center_sticker("◤", self.ce[0]),
            Self::color_corner_sticker("◢", self.cp[0], self.co[0], 0),
            Self::color_corner_sticker("◣", self.cp[0], self.co[0], 1),
            "◥".red(),
            Self::color_corner_sticker("◣", self.cp[3], self.co[3], 1),
            Self::color_corner_sticker("◥", self.cp[3], self.co[3], 2),

            Self::color_corner_sticker("◣", self.cp[1], self.co[1], 2),
            Self::color_corner_sticker("◥", self.cp[1], self.co[1], 1),
            "◣".purple(),
            Self::color_corner_sticker("◥", self.cp[0], self.co[0], 3),
            Self::color_corner_sticker("◤", self.cp[0], self.co[0], 2),
            Self::color_center_sticker("◢", self.ce[1]),
            Self::color_corner_sticker("◤", self.cp[3], self.co[3], 0),
            Self::color_corner_sticker("◢", self.cp[3], self.co[3], 3),

            Self::color_corner_sticker("◣", self.cp[5], self.co[5], 2),
            Self::color_center_sticker("◥", self.ce[2]),
            Self::color_corner_sticker("◣", self.cp[4], self.co[4], 0),
            Self::color_corner_sticker("◥", self.cp[4], self.co[4], 1),
            Self::color_corner_sticker("◤", self.cp[4], self.co[4], 2),
            Self::color_corner_sticker("◢", self.cp[4], self.co[4], 3),
            "◤".truecolor(255, 255, 0),
            Self::color_corner_sticker("◢", self.cp[5], self.co[5], 3)
        )
    }
}