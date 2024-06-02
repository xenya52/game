use crate::game_logic::Material;
//////////////////////
///External imports///
//////////////////////
use std::vec;
use crossterm::style::Color;
/////////
//Board//
/////////
pub type Board = Vec<Vec<Block>>;
#[derive(Clone)]
pub struct BlockType {
    pub durability: u32, // 3 = three hits
    pub is_passable: bool,
}
impl BlockType {
    pub fn new(_durability: u32, _is_passable: bool) -> Self {
        BlockType {
            durability: _durability,
            is_passable: _is_passable,
        }
    }
    pub fn new_predefined_set() -> Vec<BlockType> {
        let unbreakable: BlockType = BlockType::new(
            404, false,
        );
        let unbreakable_resource: BlockType = BlockType::new(
            4, false,
        );
        let breakable_resource: BlockType = BlockType::new(
            2, true,
        );
        let water: BlockType = BlockType::new(
            404, false,
        );
        let air: BlockType = BlockType::new(
            404, true,
        );

        return vec![unbreakable, unbreakable_resource, breakable_resource, water, air] 
    }
}
#[derive(Clone)]

pub struct Block {
    pub number: u32,
    pub display_ascii: char,
    pub display_color: Color,
    pub block_type: BlockType,
    pub drop: Material,

}
// Returns a pack of blocks instead of a  specific 
impl Block {
    pub fn new(_number: u32, _display_ascii: char, _display_color: Color, _block_type: BlockType, _drop: Material) -> Self {
        Block {
            number: _number,
            display_ascii: _display_ascii,
            display_color: _display_color,
            block_type: _block_type,
            drop: _drop
        }
    } 
    pub fn new_predefined_set() -> Vec<Block> {
        let block_types = BlockType:: new_predefined_set();
        let materials = Material::new_predefined_set();
        //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food, [5] Air, [6] Border, [7] Cave entrance, [8] Minion
        let stone: Block = Block::new(
            0, 
            '#',
            Color::Rgb { r: (115), g: (115), b: (115) },
            block_types[1].clone(),
            materials[0].clone()
        );
        let dirt: Block = Block::new(
            1,
             '#', 
             Color::Rgb { r: (94), g: (55), b: (25) },
            block_types[1].clone(),
            materials[3].clone(),
        );
        let water: Block = Block::new(
            2, 
            '≈',
            Color::Rgb { r: (0), g: (247), b: (255) },
            block_types[3].clone(),
            materials[4].clone(),
        );
        let wood: Block = Block::new(
            3, 
            'H',
            Color::Rgb { r: (128), g: (95), b: (57) },
            block_types[2].clone(),
            materials[1].clone(),
        );
        let food: Block = Block::new(
            4,
            'O',
            Color::Rgb { r: (255), g: (0), b: (0) },
            block_types[2].clone(),
            materials[5].clone(),
        );
        let air: Block = Block::new(
            5, 
            ' ', 
            Color::Reset,
            block_types[4].clone(),
            materials[0].clone(),
        );
        let border: Block = Block::new(
            6,
            '/', 
            Color::Reset, 
            block_types[0].clone(),
            materials[0].clone(),
        );
        let cave_entrance: Block = Block::new(
            7, 
            'o',
            Color::Rgb { r: (46), g: (46), b: (46) }, 
            block_types[0].clone(),
            materials[0].clone(),
        );
        let minion: Block = Block::new(
            8, 
            '@',
            Color::Rgb { r: (255), g: (238), b: (0) },
            block_types[0].clone(),
            materials[0].clone(),
        );
        let leaf: Block = Block::new(
            8,
            'X',
            Color::Rgb { r: (17), g: (255), b: (0) },
            block_types[0].clone(),
            materials[0].clone(),
        );
        return vec![stone,dirt,water,wood,food, air, border, cave_entrance, minion, leaf];
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
