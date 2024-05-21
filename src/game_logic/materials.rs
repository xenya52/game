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