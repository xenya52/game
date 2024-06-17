//////////////////////
///External imports///
//////////////////////
use colorized::*;
use crossterm::style::{style, Stylize, Color};
use rand::{thread_rng, Rng};

use crate::{utils::find_char_in_board, world::Block};

#[derive(Clone)] 
pub struct Inventory {
    pub items: Vec<String>,
    pub index: usize,
}
impl Inventory {
    pub fn new(_space: usize) -> Self {
        Inventory {
            items: vec!["---".to_string(); _space],
            index: 1,
        }
    }
    pub fn add(inventory: &mut Inventory, val: String) {
        for i in 0..inventory.items.len() {
            if inventory.items[i] == "---" {
                inventory.items[i] = val;
                break;
            }
        }
    }
    pub fn remove(inventory: &mut Inventory, index: usize) {
        inventory.items[index] = "---".to_string();
    }
}

#[derive(Clone)]
pub struct BasicNeeds {
    pub starve: u32,
    pub hydrate: u32,
    pub confident: u32,
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
#[derive(Clone)]
pub struct Entity<> {
    pub y: usize,
    pub x: usize,
    pub name: String,
    pub health: u32,
    pub strength: u32,
    pub actions: u64,
    pub basic_needs: BasicNeeds,
    pub inventory: Inventory,
}
impl Entity {
    pub fn new(y: usize, x: usize, name: String, health: u32, strength: u32, actions: u64, basic_needs: BasicNeeds, inventory_space: usize) -> Self {
        Entity {
            y,
            x,
            name,
            health,
            strength,
            actions,
            basic_needs,
            inventory: Inventory::new(inventory_space),
        }
    }
    pub fn block_to_inventory(entity: &mut Entity, block: Block) {
        Inventory::add(&mut entity.inventory, block.name);
    }
    pub fn show_entity_status(entity: &Entity) {
        println!("<-=-=-=-=-=-=-=->");
        println!("<Name: {}", entity.name);    
        println!("<Convinience {}", entity.basic_needs.confident);
        println!("<~~~~~~~~~~~~~~~>");
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
        print!("{}",Colors::Reset.value());
        println!("<-=-=-=-=-=-=-=->");
    }
    pub fn display_entity_inventory(entity: &mut Entity) {
        print!("<-=-=-=-=-=-=-=->\n<");
        let mut index = entity.inventory.items.len();
        while index > 0 {
            let cur_item = &entity.inventory.items[index-1];
            if cur_item != "nothing" {
                if index == entity.inventory.index {
                  let styled_content = style(cur_item)
                      .with(Color::Rgb {r: (255), g: (255), b: (255)});
                  print!("{}", styled_content)
                }
                else {
                  print!("{}. {}", index, cur_item);
                }
                if index % 3 == 0 {
                    print!("\n<")
                }
                else {
                    print!(" | ")
                }
            }
            index -= 1;
        }
        println!("\n<-=-=-=-=-=-=-=->");
    }
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
        println!(r" _   _  ______  _____  _____ ______  _ ");
        println!(r"| | | | |  _  \|_   _||  ___||  _  \| |");
        println!(r"| | | | | | | |  | |  | |__  | | | || |");
        println!(r"| | | | | | | |  | |  |  __| | | | || |");
        println!(r"| |_| | | |/ /  _| |_ | |___ | |/ / |_|");
        println!(r" \___/  |___/   \___/ \____/ |___/  (_)");
        true
    }
    else {
        false
    }
}
