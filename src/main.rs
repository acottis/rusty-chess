mod game;
use game::*;
use std::io;

fn main() {
    game_loop();
}

fn game_loop() {
    let mut board: Board = Board::init();

    board.check();

    // for i in 1..20 {
    //     board.render();
    //     if i % 2 != 0 {
    //         loop {
    //             println!("Whites turn:");
    //             let mov: (String, String) = choose_move();
    //             if board.move_piece(Colour::Light, mov.0, mov.1) == false {
    //                 println!("Invalid Move!");
    //             } else {
    //                 break;
    //             }
    //         }
    //     } else {
    //         loop {
    //             println!("Black's turn:");
    //             let mov: (String, String) = choose_move();
    //             if board.move_piece(Colour::Dark, mov.0, mov.1) == false {
    //                 println!("Invalid Move!");
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }
}

fn choose_move() -> (String, String) {
    let mut src = String::new();
    let mut dst = String::new();
    let mut n:usize;

    println!("Src:");
    loop {
        n = io::stdin().read_line(&mut src).unwrap();
        if n == 3 {
            break;
        }
        println!("Invalid source");
    }
    println!("Dst:");
    loop {
        n = io::stdin().read_line(&mut dst).unwrap();
        if n == 3 {
            break;
        }
        println!("Invalid destination");
    }
    (src, dst)
}
