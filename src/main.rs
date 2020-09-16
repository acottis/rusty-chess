mod game;

use game::*;

fn main() {
    game_loop();
}

fn game_loop() {
    let board: Board = Board::init();
    board.render();
}
