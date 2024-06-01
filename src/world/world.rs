use crate::game_logic::Materials;
//////////////////////
///External imports///
//////////////////////
use std::vec;
/////////
//Board//
/////////
pub type Board = Vec<Vec<Block>>;
#[derive(Clone)]
pub struct BlockType {
    pub name: String,
    pub durability: u32, // 3 = three hits
    pub is_passable: bool,
}
impl BlockType {
    pub fn new(_name: String, _durability: u32, _is_passable: bool) -> Self {
        BlockType {
            name: _name,
            durability: _durability,
            is_passable: _is_passable,
        }
    }
    pub fn new_predefined_set() -> Vec<BlockType> {
        //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food, [5] Air, [6] Unbreakable
        let stone: BlockType = BlockType::new(
            "Stone".to_string(), 
            4, true,
        );
        let dirt: BlockType = BlockType::new(
            "dirt".to_string(), 
            2, true,
        );
        let water: BlockType = BlockType::new(
            "water".to_string(), 
            0, false,
        );
        let wood: BlockType = BlockType::new(
            "wood".to_string(), 
            2, true,
        );
        let food: BlockType = BlockType::new(
            "food".to_string(), 
            2, true,
        );
        let air: BlockType = BlockType::new(
            "air".to_string(), 
            0, true,
        );
        let unbreakable: BlockType = BlockType::new(
            "unbreakable".to_string(), 
            0, false,
        );

        return vec![stone, dirt, water, wood, food, air, unbreakable] 
    }
}
#[derive(Clone)]

pub struct Block {
    pub number: u32,
    pub display_ascii: char,
    pub display_color: String,
    pub block_type: BlockType,
    pub drop: Materials,

}
// Returns a pack of blocks instead of a  specific 
impl Block {
    pub fn new(_number: u32, _display_ascii: char, _display_color: String, _block_type: BlockType, _drop: Materials) -> Self {
        Block {
            number: _number,
            display_ascii: _display_ascii,
            display_color: _display_color,
            block_type: _block_type,
            drop: _drop
        }
    } 
    pub fn new_predefined_set() -> Vec<Block> {
        let temp = BlockType:: new_predefined_set();
        //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food, [5] Air, [6] Border, [7] Cave entrance, [8] Minion
        let stone: Block = Block::new(
            0, 
            '#',
            "gray".to_string(), 
            temp[0].clone(),
            Materials::new(0, 1)
        );
        let dirt: Block = Block::new(
            1,
             '#', 
             "brown".to_string(),
            temp[1].clone(),
            Materials::new(0, 0)
        );
        let water: Block = Block::new(
            2, 
            '≈',
            "blue".to_string(),
            temp[2].clone(),
            Materials::new(0, 0)
        );
        let wood: Block = Block::new(
            3, 
            'H',
            "brown".to_string(), 
            temp[3].clone(),
            Materials::new(0, 0)
        );
        let food: Block = Block::new(
            4,
            'O',
            "red".to_string(),
            temp[4].clone(),
            Materials::new(0, 0)
        );
        let air: Block = Block::new(
            5, 
            ' ', 
            "".to_string(),
             temp[5].clone(),
             Materials::new(0, 0)
        );
        let border: Block = Block::new(
            6,
            '/', 
            "".to_string(), 
        temp[6].clone(),
        Materials::new(0, 0)
        );
        let cave_entrance: Block = Block::new(
            7, 
            'o',
            "black".to_string(), 
            temp[6].clone(),
            Materials::new(0, 0)
        );
        let minion: Block = Block::new(
            8, 
            '@', 
            "yellow".to_string(), 
            temp[6].clone(),
            Materials::new(0, 0)
        );
        return vec![stone,dirt,water,wood,food, air, border, cave_entrance, minion];
    }
}
pub struct World {
    pub overworld: Board,
    pub cave: Board,
    pub is_on_overworld: bool
}
impl World {
    pub fn new(_overworld: Vec<Vec<Block>>, _cave: Vec<Vec<Block>>) -> Self {
        World {
            overworld: _overworld,
            cave: _cave,
            is_on_overworld: true
        }
    }
}