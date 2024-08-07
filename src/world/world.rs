//////////////////////
///External imports///
//////////////////////
use std::vec;
use crossterm::style::Color;

pub type Board = Vec<Vec<Block>>;
#[derive(Clone, PartialEq)]
pub struct BlockType {
  pub durability: u32,
  pub is_passable: bool,
}
impl BlockType {
  pub fn new(durability: u32, is_passable: bool) -> Self {
    BlockType {
      durability,
      is_passable,
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
#[derive(Clone, PartialEq)]
pub struct Block {
  pub name: String,
  pub display_ascii: char,
  pub display_color: Color,
  pub block_type: BlockType,
}
impl Block {
  pub fn new(name: String, display_ascii: char, display_color: Color, block_type: BlockType) -> Self {
    Block {
      name,
      display_ascii,
      display_color,
      block_type,
    }
  } 
  pub fn new_predefined_set() -> Vec<Block> {
    let block_types = BlockType:: new_predefined_set();
    //[0] Stone, [1] Dirt, [2] Water, [3] Wood, [4] Food, [5] Air, [6] Border, [7] Cave entrance, [8] Minion
    let stone: Block = Block::new(
      "Stone".to_string(),
      '#',
      Color::Rgb { r: (115), g: (115), b: (115) },
      block_types[1].clone(),
    );
    let dirt: Block = Block::new(
      "Dirt".to_string(),
      '#', 
      Color::Rgb { r: (94), g: (55), b: (25) },
      block_types[1].clone(),
    );
    let water: Block = Block::new(
      "Water".to_string(),
      '≈',
      Color::Rgb { r: (0), g: (247), b: (255) },
      block_types[1].clone(),
    );
    let wood: Block = Block::new(
       "Wood".to_string(),
       'H',
       Color::Rgb { r: (128), g: (95), b: (57) },
       block_types[2].clone(),
    );
    let food: Block = Block::new(
      "Food".to_string(),
      'O',
      Color::Rgb { r: (255), g: (0), b: (0) },
      block_types[2].clone(),
    );
    let air: Block = Block::new(
      "Air".to_string(),
      ' ', 
      Color::Reset,
      block_types[4].clone(),
    );
    let border: Block = Block::new(
      "Border".to_string(),
      '/', 
      Color::Reset, 
      block_types[0].clone(),
    );
    let cave_entrance: Block = Block::new(
      "Cave_Entrance".to_string(),
      'o',
      Color::Rgb { r: (255), g: (255), b: (255) }, 
      block_types[0].clone(),
    );
    let minion: Block = Block::new(
       "Minion".to_string(),
       '@',
       Color::Rgb { r: (255), g: (238), b: (0) },
       block_types[0].clone(),
    );
    let leaf: Block = Block::new(
      "Leaf".to_string(),
      'X',
      Color::Rgb { r: (17), g: (255), b: (0) },
      block_types[2].clone(),
    );
    return vec![stone,dirt,water,wood,food, air, border, cave_entrance, minion, leaf];
  }
}
pub struct World {
  pub overworld: Board,
  pub cave: Board,
}
impl World {
  pub fn new(overworld: Vec<Vec<Block>>, cave: Vec<Vec<Block>>) -> Self {
    World {
      overworld,
      cave,
    }
  }
}
