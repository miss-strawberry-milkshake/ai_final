use macroquad::prelude::*;
use std::{thread, time};
mod board_utils;
mod alpha_beta;
mod general_utils;
mod graphics;
mod rl;

#[macroquad::main("Tuck Checkers")]
async fn main() {

    let mut epsilon: f32 = 1.;

    // while epsilon < 1. {
    loop {

    let mut board: [[i16; 8]; 8] = board_utils::generate_board();

    let mut white_turn: bool = true;


    let mut white_weight_vector:[f32; 10] = [0.; 10];
    let mut red_weight_vector:[f32; 10] = [0.; 10];
    rl::read_weight_file(&mut white_weight_vector, true);
    rl::read_weight_file(&mut red_weight_vector, false);

    // main game loop
    loop {
        //draw the current state of the board and the peices
        graphics::draw_board();
        graphics::draw_peices(board).await;
        thread::sleep(time::Duration::from_millis(500));

        //check if game is over
        let possible_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = board_utils::generate_moves(&mut board, white_turn);
        if possible_moves.len() == 0 && white_turn{
            println!("RED WINS!!");
            break;
        }
        if possible_moves.len() == 0 && !white_turn{
            println!("WHITE WINS!!");
            break;
        }

        if white_turn{
            // alpha_beta::alpha_beta(&mut board).await;
            rl::rl_learner(&mut board, &mut white_weight_vector, true, epsilon);
        } else {
            // rl::rl_learner(&mut board, &mut red_weight_vector,false, epsilon);

            // print all the moves if avalible
            if white_turn{println!("WHITE TURN!");} else {println!("RED TURN!");}
            for print_all_moves in 0..possible_moves.len(){
                    println!("{}: {:?}", print_all_moves, (possible_moves[print_all_moves]));
            }

            //get user input for the move they wanna try
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let user_move_int: i16 = line.trim().parse().unwrap();
            let user_move: usize = user_move_int as usize;

            println!("selecting move number {}, as {:?}", user_move, possible_moves[user_move]);

            board_utils::do_move(&mut board, possible_moves[user_move]);
        }

        white_turn = !white_turn;
    }
        epsilon += 0.0000001;
        println!("epsilon = {}", epsilon);
        rl::write_weight_file(white_weight_vector, true);
        rl::write_weight_file(red_weight_vector, false);
    }
}