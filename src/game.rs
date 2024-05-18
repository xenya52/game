use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode}
};
use std::char;
pub use rand::{thread_rng, Rng, seq::SliceRandom};
use colorized::Colors;

//Own stuff lib.rs
use crate::{board::{self, init_cave}, find_char_in_board, Board, World};

//////////////////////
//General game logic//
//////////////////////
pub fn game_over(input: char, player: Entity) -> bool {
    if input == 'q' {
        return true;
    }
    else if dead_entity(player) {
        return true;
    }
    return  false;
}
#[derive(Clone, Copy, Debug)]
pub struct Materials {
    wood: u32,
    stone: u32,
}

impl Materials {
    pub fn new(wood: u32, stone: u32) -> Self {
        Materials { wood, stone }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BasicNeeds {
    starve: u32,
    hydrate: u32,
    confident: u32,
}

impl BasicNeeds {
    pub fn new(starve: u32, hydrate: u32, confident: u32) -> Self {
        BasicNeeds {
            starve,
            hydrate,
            confident,
        }
    }
}
//////////
//Entity//
//////////
#[derive(Clone, Copy)]
pub struct Entity {
    health: u32,
    strength: u32,
    actions: u64,
    basic_needs: BasicNeeds,
    materials: Materials
}
impl Entity {
    pub fn new(health: u32, strength: u32, actions: u64, basic_needs: BasicNeeds, materials: Materials) -> Self {
        Entity {
            health,
            strength,
            actions,
            basic_needs,
            materials,
        }
    }
}
pub fn show_entity_status(entity: &Entity) {
    println!("<-=-=-=-=-=-=-=->");
    println!("<Health: {}", entity.health);
    println!("<Strengh: {}", entity.strength);
    println!("<Total actions: {}", entity.actions);
    println!("<~~~~~~~~~~~~~~~>");
    if entity.basic_needs.starve <= 4 {
        print!("{}",Colors::BrightRedFg.value());
    }
    else if entity.basic_needs.starve <= 8 {
        print!("{}",Colors::BrightYellowFg.value());
    }
    println!("<Starvation {}", entity.basic_needs.starve);
    print!("{}",Colors::Reset.value());
    if entity.basic_needs.hydrate <= 4 {
        print!("{}",Colors::BrightRedFg.value());
    }
    else if entity.basic_needs.hydrate <= 8 {
        print!("{}",Colors::BrightYellowFg.value());
    }
    println!("<Hydration {}", entity.basic_needs.hydrate);
    print!("{}",Colors::Reset.value());
    if entity.basic_needs.confident <= 4 {
        print!("{}",Colors::BrightRedFg.value());
    }
    else if entity.basic_needs.confident <= 8 {
        print!("{}",Colors::BrightYellowFg.value());
    }
    println!("<Convinience {}", entity.basic_needs.confident);
    print!("{}",Colors::Reset.value());
    println!("<~~~~~~~~~~~~~~~>");
    println!("<Wood {}", entity.materials.wood);
    println!("<Stone {}", entity.materials.stone);
    println!("<-=-=-=-=-=-=-=->");
}
pub fn entity_moved(entity: &mut Entity) {
    entity.actions += 1;
    entity.basic_needs.starve -= if thread_rng().gen_bool(0.9) { 0 } else { 1 };
    entity.basic_needs.hydrate -= if thread_rng().gen_bool(0.9) { 0 } else { 2 };
}
pub fn dead_entity(entity: Entity) -> bool 
{
    if entity.health <= 0 ||
       entity.basic_needs.hydrate <= 0 ||
       entity.basic_needs.starve <= 0 {
        true
    }
    else {
        false
    }
}
/////////////////
//Eniemy entity//
/////////////////
pub struct Eniemy {
    health: u32,
    strength: u32,
}
impl Eniemy {
    pub fn new(_health: u32, _strength: u32,) -> Self {
        Eniemy {
            health: _health,
            strength: _strength
        }
    }
}
//////////////////////////////
//General movement functions//
//////////////////////////////
fn move_up(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        // Swap the character with the element above it
        let temp = board[y_usize - 1][x_usize];
        board[y_usize - 1][x_usize] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}
fn move_down(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize + 1][x_usize];
        board[y_usize + 1][x_usize] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}

fn move_right(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize][x_usize + 1];
        board[y_usize][x_usize + 1] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}

fn move_left(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize][x_usize - 1];
        board[y_usize][x_usize - 1] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}
fn move_possibilites(board: &mut Board, usr_input:char, coordinates: &Vec<(u32, u32)>, entity: &mut Entity) -> bool {
    if let Some((x, y)) = coordinates.get(0) {
        if *x > 0 {
            let mut x_usize = *x as usize;
            let mut y_usize = *y as usize;
            
            match usr_input {
                'w' => y_usize -= 1,
                'a' => x_usize -= 1,
                's' => y_usize += 1,
                'd' => x_usize += 1,
                _ => println!("Error")
            }
            //Action for stone
            if board[y_usize][x_usize] == 'x' || board[y_usize][x_usize] == 'X' {
                entity.materials.stone += 1;
            }
            //Action for wood
            if board[y_usize][x_usize] == '|' {
                entity.materials.wood += 1;
                board[y_usize][x_usize] = '#';
                return true;
            }
            //Action for water
            if board[y_usize][x_usize] == '~' || board[y_usize][x_usize] == '≈' {
                entity.basic_needs.hydrate = 10;
            }
            //Action for food
            if board[y_usize][x_usize] == '+' {
                entity.basic_needs.starve = 10;
                board[y_usize][x_usize] = '#';
                return true;
            }
            //Action for hitting the eniemy_entity
            if board[y_usize][x_usize] == 'ö' {
                entity.health -= 1;
            }
            if board[y_usize][x_usize] == 'o' {
                init_cave();
            }
            //Action for default grass
            if board[y_usize][x_usize] == '#' {
                return true;
            }
        }
    }
    return false;
}
////////////////
//User actions//
////////////////
pub fn get_user_input() -> char {
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut input_char = ' ';
    loop {
        if let Event::Key(key_event) = read().expect("Failed to read event") {
            match key_event.code {
                KeyCode::Char(c) => {
                    input_char = c;
                    break;
                }
                _ => continue,
            }
        }
    }

    // Deaktiviere den Raw-Modus, bevor das Programm endet
    disable_raw_mode().expect("Failed to disable raw mode");
    input_char
}
pub fn handle_input(usr_input: char, board: &mut Board, entity: &mut Entity) {
    let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
    if move_possibilites(board, usr_input, &coordinates, entity) {
        match usr_input {
            'w' => move_up(coordinates ,board),
            'a' => move_left(coordinates, board),
            's' => move_down(coordinates, board),
            'd' => move_right(coordinates, board),
            _ => println!("Error")
        }
    }
}
////////////////////
//Preditor actions//
////////////////////
pub fn move_preditor(board: &mut Board, entity: &mut Eniemy) {
    // let possible_move: bool = rand::thread_rng().gen_bool(0.5);
    // println!("{}",possible_move);
    // if possible_move
    //  {
    //     let co_preditor: Vec<(u32,u32)> = find_char_in_board(board, 'ö');
    //     let co_player: Vec<(u32,u32)> = find_char_in_board(board, '@');
    //     // Assuming both vectors are guaranteed to have at least one element.
    //     // You should add checks to ensure they are not empty to avoid runtime panics.
    //     let resultx: i32 = co_preditor[0].0 as i32 - co_player[0].0 as i32; // Subtract x coordinates
    //     let resulty: i32 = co_preditor[0].1 as i32 - co_player[0].1 as i32; // Subtract y coordinates
    //     if resultx > 0 {
    //         if move_possibilites(board, 'a', &co_preditor, entity) {
    //             move_left(co_preditor, board);
    //         }
    //     }
    //     else if resultx < 0 {
    //         if move_possibilites(board, 'd', &co_preditor, entity) {
    //             move_right(co_preditor, board);
    //         }
    //     }
    //     else if resulty < 0 {
    //         if move_possibilites(board, 's', &co_preditor, entity) {
    //             move_down(co_preditor, board);
    //         }
    //     }
    //     else if resulty > 0 {
    //         if move_possibilites(board, 'w', &co_preditor, entity) {
    //             move_up(co_preditor, board);
    //         }
    //     }
    //  }
}