use std::{u8::MAX, vec};

use crate::game_logic::Materials;

/////////
//Board//
/////////
pub type Board = Vec<Vec<char>>;
pub type BBoard = Vec<Vec<Block>>;
#[derive(Clone)]
pub struct Block_Type {
    pub name: String,
    pub durability: u32, // 3 = three hits
    pub drop: Vec<Materials>
}
impl Block_Type {
    pub fn new(_name: String, _durability: u32, _drop: Vec<Materials>) -> Self {
        Block_Type {
            name: _name,
            durability: _durability,
            drop: _drop
        }
    }
    pub fn new_predefined_set() -> Vec<Block_Type> {
        //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food
        let stone: Block_Type = Block_Type::new(
            "Stone".to_string(), 4, 
            vec![Materials::new(0, 1)]
        );
        let dirt: Block_Type = Block_Type::new(
            "dirt".to_string(), 2,
            vec![Materials::new(0, 0)]
        );
        let water: Block_Type = Block_Type::new(
            "water".to_string(), 999, 
            vec![Materials::new(0, 0)]
        );
        let wood: Block_Type = Block_Type::new(
            "wood".to_string(), 2, 
            vec![Materials::new(1, 0)]
        );
        let food: Block_Type = Block_Type::new(
            "food".to_string(), 2, 
            vec![Materials::new(0, 0)]
        );
        let air: Block_Type = Block_Type::new(
            "air".to_string(), 0, 
            vec![Materials::new(0, 0)]
        );

        return vec![stone, dirt, water, wood, food, air] 
    }
}
#[derive(Clone)]

pub struct Block {
    pub number: u32,
    pub display_ascii: char,
    pub display_color: String,
    pub block_type: Block_Type
}
// Returns a pack of blocks instead of a  specific 
impl Block {
    pub fn new(_number: u32, _display_ascii: char, _display_color: String, _block_type: Block_Type) -> Self {
        Block {
            number: _number,
            display_ascii: _display_ascii,
            display_color: _display_color,
            block_type: _block_type
        }
    } 
    pub fn new_predefined_set() -> Vec<Block> {
        let temp = Block_Type:: new_predefined_set();
        //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food
        let stone: Block = Block::new(
            0, 
            '#',
            "gray".to_string(), 
            temp[0].clone()
        );
        let dirt: Block = Block::new(
            1,
             '#', 
             "brown".to_string(),
            temp[1].clone()
        );
        let water: Block = Block::new(
            2, 
            '≈',
            "blue".to_string(),
            temp[2].clone()
        );
        let wood: Block = Block::new(
            3, 
            'H',
            "brown".to_string(), 
            temp[3].clone()
        );
        let food: Block = Block::new(
            4,
            'O',
            "red".to_string(),
            temp[4].clone()
        );
        let air: Block = Block::new(
            5, 
            ' ', 
            "".to_string(),
             temp[5].clone()
        );
        return vec![stone,dirt,water,wood,food, air];
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