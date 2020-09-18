mod game;
use game::*;
use std::io;

fn main() {
    game_loop();
}

fn game_loop() {
    let mut board: Board = Board::init();

    board.check(Colour::Light);

    for i in 1..20 {
        board.render();
        if i % 2 != 0 {
            loop {
                println!("Whites turn:");
                let mov: (String, String) = choose_move();
                if board.move_piece(Colour::Light, mov.0, mov.1) == false {
                    println!("Invalid Move!");
                } else {
                    break;
                }
            }
            if board.check(Colour::Light){
                println!("Check!");
            }
        } else {
            loop {
                println!("Black's turn:");
                let mov: (String, String) = choose_move();
                if board.move_piece(Colour::Dark, mov.0, mov.1) == false {
                    println!("Invalid Move!");
                } else {
                    break;
                }
            }
            if board.check(Colour::Dark){
                println!("Check!");
            }
        }
    }
}

fn choose_move() -> (String, String) {
    let mut src = String::new();
    let mut dst = String::new();

    println!("Src:");
    io::stdin().read_line(&mut src).unwrap();
    println!("Dst:");
    io::stdin().read_line(&mut dst).unwrap();
    (src, dst)
}
