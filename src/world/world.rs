/////////
//Board//
/////////
pub type Board = Vec<Vec<char>>;
pub struct World {
    pub overworld: Vec<Vec<char>>,
    pub cave: Vec<Vec<char>>,
    pub is_on_overworld: bool
}
impl World {
    pub fn new(_overworld: Vec<Vec<char>>, _cave: Vec<Vec<char>>) -> Self {
        World {
            overworld: _overworld,
            cave: _cave,
            is_on_overworld: true
        }
    }
}