use crate::game_logic::Material;
//////////////////////
///External imports///
//////////////////////
use colorized::*;
use rand::{thread_rng, Rng};

#[derive(Clone)] 
pub struct Inventory {
    pub materials: Vec<Material>,
    pub space: u32
}
impl Inventory {
    pub fn new(_space: u32) -> Inventory {
        Inventory {
            space: _space,
            materials: Vec::new(),
        }
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
    // pub name: str, TODO
    pub name: String,
    pub health: u32,
    pub strength: u32,
    pub actions: u64,
    pub basic_needs: BasicNeeds,
    pub inventory: Inventory,
    pub turns: u32,
}
impl Entity {
    pub fn new(name: String, health: u32, strength: u32, actions: u64, basic_needs: BasicNeeds, inventory_space: u32) -> Self {
        Entity {
            name,
            health,
            strength,
            actions,
            basic_needs,
            inventory: Inventory::new(inventory_space),
            turns: 0,
        }
    }
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
