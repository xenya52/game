fn place_player(board: &mut Board) {
  board[8][8] = '@';
}
fn place_eniemy(board: &mut Board) {
  let c:Vec<usize> = get_rdm_xy(board);
  board[c[0]][c[1]] = 'ö';
}