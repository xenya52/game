#[derive(Clone, Copy, Debug)]
pub struct Materials {
    pub wood: u32,
    pub stone: u32,
}

impl Materials {
    pub fn new(wood: u32, stone: u32) -> Self {
        Materials { wood, stone }
    }
}