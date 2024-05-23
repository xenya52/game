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