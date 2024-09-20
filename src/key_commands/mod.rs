mod up;
mod down;
mod left;
mod right;

pub use up::UpCommand;
pub use down::DownCommand;
pub use left::LeftCommand;
pub use right::RightCommand;

use crate::game_logic::Player;

pub trait Command {
    fn execute(&mut self, player: &mut Player) -> bool;
    fn undo(&mut self, player: &mut Player);
}