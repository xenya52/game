use crate::world::Board;
use crate::game_logic::Player;
use crate::utils::create_rendered_board;
//////////////////////
///External imports///
//////////////////////
use crossterm::style::{style, Stylize};

pub fn print_given_board(given_board: &mut Board, player: &mut Player) {
  let mut rendered_board = create_rendered_board(given_board, player, player.render_distance);
  for (_y, row) in rendered_board.iter_mut().enumerate() {
    for (_x, col) in row.iter_mut().enumerate() {
      let styled_content = style(col.display_ascii)
        .with(col.display_color);
      if player.render_distance_y == _y && player.render_distance_x == _x {
        print!("{}", styled_content.on_yellow())
      }
      else {
        print!("{}", styled_content);
      }
    }
  println!();
  }
}
pub fn print_keybindings() {
  println!("Movement [w] [a] [s] [d]");
  println!("Inventory [i]");
  println!("Get/Out entity: [r]")
}
