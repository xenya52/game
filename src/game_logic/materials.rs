#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub amount: u32
}

impl Material {
    pub fn new(name: String) -> Self {
        Material { 
            name,
            amount: 1    
        }
    }
    pub fn new_predefined_set() -> Vec<Material> {
        let nothing: Material = Material::new("".to_string());
        let stone: Material = Material::new("stone".to_string());
        let wood: Material = Material::new("wood".to_string());
        let dirt: Material = Material::new("dirt".to_string());
        let water: Material = Material::new("water".to_string());
        let food: Material = Material::new("food".to_string());
        let leaf: Material = Material::new("leaf".to_string());
        return vec![nothing, wood, stone, dirt, water, food, leaf]
    }
}
