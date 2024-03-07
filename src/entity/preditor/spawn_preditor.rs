
fn set_preditor_in_board(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = 'ö';
}