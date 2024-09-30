mod up;
mod down;
mod left;
mod right;
mod get_entity;

pub use up::UpCommand;
pub use down::DownCommand;
pub use left::LeftCommand;
pub use right::RightCommand;
pub use get_entity::GetEntityCommand;

use crate::game_logic::Player;

pub trait Command {
    fn execute(player: &mut Player) -> bool;
    fn undo(player: &mut Player);
}