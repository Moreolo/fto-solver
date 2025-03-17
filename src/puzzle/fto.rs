use std::fmt;

use crate::state::{statefto::StateFTO, unwrap_turn, State};

use super::Puzzle;


impl Puzzle for StateFTO {
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
            "U" => self.apply_turn(8),
            "U'" => self.apply_turn(9),
            "F" => self.apply_turn(10),
            "F'" => self.apply_turn(11),
            "BL" => self.apply_turn(12),
            "BL'" => self.apply_turn(13),
            "BR" => self.apply_turn(14),
            "BR'" => self.apply_turn(15),
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
            4 => if cw {"U"} else {"U'"},
            5 => if cw {"F"} else {"F'"},
            6 => if cw {"BL"} else {"BL'"},
            7 => if cw {"BR"} else {"BR'"},
            _ => ""
        }
    }
}

/*
{}{} {}{} {} /{} |{} \\{} {}{} {}{}\n
{}{} {} /{} {}{} |{}{} {} \\{} {}{}\n
{} /{} {}{} {}{} |{}{} {}{} {} \\{}\n
-----------------------\n
{} \\{} {}{} {}{} |{}{} {}{} {} /{}\n
{}{} {} \\{} {}{} |{}{} {} /{} {}{}\n
{}{} {}{} {} \\{} |{} /{} {}{} {}{}

*/

impl fmt::Display for StateFTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} {}{} {} /{} |{} \\{} {}{} {}{}\n{}{} {} /{} {}{} |{}{} {} \\{} {}{}\n{} /{} {}{} {}{} |{}{} {}{} {} \\{}\n-----------------------\n{} \\{} {}{} {}{} |{}{} {}{} {} /{}\n{}{} {} \\{} {}{} |{}{} {} /{} {}{}\n{}{} {}{} {} \\{} |{} /{} {}{} {}{}",
            // Row 1
            Self::color_corner_sticker("◤", self.cp[5], self.co[5], 1),
            Self::color_center_sticker("◢", self.ce2[3] + 4),
            Self::color_edge_sticker("◤", self.ep[9], false),
            Self::color_center_sticker("◢", self.ce2[4] + 4),
            Self::color_corner_sticker("◤", self.cp[2], self.co[2], 1),
            Self::color_corner_sticker("◢", self.cp[2], self.co[2], 0),
            Self::color_corner_sticker("◣", self.cp[2], self.co[2], 3),
            Self::color_corner_sticker("◥", self.cp[2], self.co[2], 2),
            Self::color_center_sticker("◣", self.ce1[9]),
            Self::color_edge_sticker("◥", self.ep[9], true),
            Self::color_center_sticker("◣", self.ce1[10]),
            Self::color_corner_sticker("◥", self.cp[5], self.co[5], 0),
            // Row 2
            Self::color_edge_sticker("◤", self.ep[8], false),
            Self::color_center_sticker("◢", self.ce2[5] + 4),
            Self::color_edge_sticker("◤", self.ep[1], false),
            Self::color_edge_sticker("◢", self.ep[1], true),
            Self::color_center_sticker("◤", self.ce1[2]),
            Self::color_edge_sticker("◢", self.ep[2], true),
            Self::color_edge_sticker("◣", self.ep[2], false),
            Self::color_center_sticker("◥", self.ce2[11] + 4),
            Self::color_edge_sticker("◣", self.ep[11], false),
            Self::color_edge_sticker("◥", self.ep[11], true),
            Self::color_center_sticker("◣", self.ce1[11]),
            Self::color_edge_sticker("◥", self.ep[10], true),
            // Row 3
            Self::color_corner_sticker("◤", self.cp[1], self.co[1], 3),
            Self::color_corner_sticker("◢", self.cp[1], self.co[1], 0),
            Self::color_center_sticker("◤", self.ce1[1]),
            Self::color_edge_sticker("◢", self.ep[0], true),
            Self::color_center_sticker("◤", self.ce1[0]),
            Self::color_corner_sticker("◢", self.cp[0], self.co[0], 0),
            Self::color_corner_sticker("◣", self.cp[0], self.co[0], 1),
            Self::color_center_sticker("◥", self.ce2[10] + 4),
            Self::color_edge_sticker("◣", self.ep[3], false),
            Self::color_center_sticker("◥", self.ce2[9] + 4),
            Self::color_corner_sticker("◣", self.cp[3], self.co[3], 1),
            Self::color_corner_sticker("◥", self.cp[3], self.co[3], 2),
            // Row 4
            Self::color_corner_sticker("◣", self.cp[1], self.co[1], 2),
            Self::color_corner_sticker("◥", self.cp[1], self.co[1], 1),
            Self::color_center_sticker("◣", self.ce2[7] + 4),
            Self::color_edge_sticker("◥", self.ep[0], false),
            Self::color_center_sticker("◣", self.ce2[8] + 4),
            Self::color_corner_sticker("◥", self.cp[0], self.co[0], 3),
            Self::color_corner_sticker("◤", self.cp[0], self.co[0], 2),
            Self::color_center_sticker("◢", self.ce1[3]),
            Self::color_edge_sticker("◤", self.ep[3], true),
            Self::color_center_sticker("◢", self.ce1[4]),
            Self::color_corner_sticker("◤", self.cp[3], self.co[3], 0),
            Self::color_corner_sticker("◢", self.cp[3], self.co[3], 3),
            // Row 5
            Self::color_edge_sticker("◣", self.ep[8], true),
            Self::color_center_sticker("◥", self.ce1[6]),
            Self::color_edge_sticker("◣", self.ep[6], true),
            Self::color_edge_sticker("◥", self.ep[6], false),
            Self::color_center_sticker("◣", self.ce2[6] + 4),
            Self::color_edge_sticker("◥", self.ep[5], false),
            Self::color_edge_sticker("◤", self.ep[5], true),
            Self::color_center_sticker("◢", self.ce1[5]),
            Self::color_edge_sticker("◤", self.ep[4], true),
            Self::color_edge_sticker("◢", self.ep[4], false),
            Self::color_center_sticker("◤", self.ce2[2] + 4),
            Self::color_edge_sticker("◢", self.ep[10], false),
            // Row 6
            Self::color_corner_sticker("◣", self.cp[5], self.co[5], 2),
            Self::color_center_sticker("◥", self.ce1[8]),
            Self::color_edge_sticker("◣", self.ep[7], true),
            Self::color_center_sticker("◥", self.ce1[7]),
            Self::color_corner_sticker("◣", self.cp[4], self.co[4], 0),
            Self::color_corner_sticker("◥", self.cp[4], self.co[4], 1),
            Self::color_corner_sticker("◤", self.cp[4], self.co[4], 2),
            Self::color_corner_sticker("◢", self.cp[4], self.co[4], 3),
            Self::color_center_sticker("◤", self.ce2[1] + 4),
            Self::color_edge_sticker("◢", self.ep[7], false),
            Self::color_center_sticker("◤", self.ce2[0] + 4),
            Self::color_corner_sticker("◢", self.cp[5], self.co[5], 3)
        )
    }
}