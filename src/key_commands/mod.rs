mod up;
mod down;
mod left;
mod right;
mod inventory;

pub use up::UpCommand;
pub use down::DownCommand;
pub use left::LeftCommand;
pub use right::RightCommand;

pub trait Command {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app: &mut cursive::Cursive);
}