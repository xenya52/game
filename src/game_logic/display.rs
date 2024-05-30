use crate::world::Board;
//////////////////////
///External imports///
//////////////////////
pub use colorized::*;

/////////////////
//Print a board//
/////////////////
pub fn print_overworld(given_board: &mut Board) {
    for (y, row) in given_board.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            print!("{}", col.display_ascii);
        }
        println!();
    }
//   for (i, row) in given_board.iter().enumerate() {
//       for (j,row) in row.iter().enumerate() {
//           if row == &'@' {
//               print!("{}",Colors::BrightYellowFg.value());
//           }
//           else if row == &'ö' {
//               print!("{}",Colors::WhiteFg.value())
//           }
//           else if row == &'x' || row == &'X' {
//               print!("{}",Colors::BrightBlackFg.value());
//           }
//           else if row == &'~' {
//               print!("{}", Colors::BrightBlueFg.value());
//           }
//           else if row == &'+' || row == &'|' {
//               print!("{}", Colors::RedFg.value());
//           }
//           else {
//               print!("{}",Colors::GreenFg.value());
//           }
//           print!("{}", row);
//           print!("{}",Colors::Reset.value());
//       }
//       println!();
//   }
}
pub fn print_cave(board: &mut Board) {
//   for (i, row) in board.iter().enumerate() {
//       for (j,row) in row.iter().enumerate() {
//           if row == &'@' {
//               print!("{}",Colors::BrightYellowFg.value());
//           }
//           else if row == &'x' || row == &'X' {
//               print!("{}",Colors::BrightWhiteFg.value());
//           }
//           else if row == &'~' {
//               print!("{}", Colors::BrightBlueFg.value());
//           }
//           else {
//               print!("{}",Colors::BrightBlackFg.value());
//           }
//           print!("{}", row);
//           print!("{}",Colors::Reset.value());
//       }
//       println!();
//   }
}