use std::collections::btree_map::IterMut;

use crate::game_logic::Materials;

/////////
//Board//
/////////
pub type Board = Vec<Vec<char>>;
pub type BBoard = Vec<Vec<Block>>;
pub struct Block_Type {
    pub name: String,
    pub durability: u32, // 3 = three hits
    pub drop: Vec<Materials>
}
pub struct Block {
    pub number: u32,
    pub display_ascii: char,
    pub display_color: String
}
impl Block {
    pub fn new(number: u32, _display_ascii: char, _display_color: String) -> Self {
        Block {
            number: number,
            display_ascii: _display_ascii,
            display_color: _display_color
        }
    }
}
pub struct World {
    pub overworld: Vec<Vec<char>>,
    pub cave: Vec<Vec<char>>,
    pub is_on_overworld: bool
}
impl World {
    pub fn new(_overworld: Vec<Vec<char>>, _cave: Vec<Vec<char>>) -> Self {
        World {
            overworld: _overworld,
            cave: _cave,
            is_on_overworld: true
        }
    }
}