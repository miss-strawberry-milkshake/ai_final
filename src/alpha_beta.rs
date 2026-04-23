use macroquad::prelude::*;
use crate::board_utils;
use crate::general_utils;
// use std::{thread, time};
// use crate::graphics;
use async_recursion::async_recursion;

const POS_INF: f64 = std::f64::INFINITY;
const NEG_INF: f64 = std::f64::NEG_INFINITY;
const DEPTH: i16 = 10;

pub async fn alpha_beta(board: &mut [[i16; 8]; 8]){
    
    let possible_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = board_utils::generate_moves(board, true);
    let mut best_value: f64 = NEG_INF;

    if possible_moves.len() != 0{
        
    let mut best_move:(i16,i16,i16,i16,i16,i16,i16) = possible_moves[0];

    //all of the junk logic in this for loop is to find the actual move that gave us the highest evaluation
    for a in possible_moves{

        let mut temp_clone:[[i16; 8]; 8] = board.clone();
        board_utils::do_move(&mut temp_clone, a);
        let step_down_eval: f64 = min_player(temp_clone, NEG_INF, POS_INF, DEPTH, false).await;

        if step_down_eval > best_value{
            best_move = a;
            best_value = step_down_eval;
        }
    }
    println!("final value = {}", best_value);

    board_utils::do_move(board, best_move);

    }
}

//max player
#[async_recursion]
async fn max_player(mut board: [[i16; 8]; 8], mut alpha: f64, beta: f64, depth: i16, white_turn: bool) -> f64{

    // graphics::draw_board();
    // graphics::draw_peices(board).await;
    // thread::sleep(time::Duration::from_millis(500));

    let mut v: f64 = NEG_INF;
    let possible_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = board_utils::generate_moves(&mut board, true);
    if (possible_moves.len() == 0) || depth == 0{
        return board_utils::evaluate(&board);
    }

    for a in possible_moves{
        v = general_utils::maximum(v,min_player(board_utils::do_move(&mut board.clone(), a),alpha.clone(),beta.clone(),depth-1, !white_turn).await);
        if v >= beta as f64{
            return v
        }
        alpha = general_utils::maximum(alpha, v);
    }
    return v
}

//min player
#[async_recursion]
async fn min_player(mut board: [[i16; 8]; 8], alpha: f64, mut beta: f64, depth: i16, white_turn: bool) -> f64{

    // graphics::draw_board();
    // graphics::draw_peices(board).await;
    // thread::sleep(time::Duration::from_millis(500));

    let mut v: f64 = POS_INF;
    let possible_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = board_utils::generate_moves(&mut board, false);
    if (possible_moves.len() == 0)  || depth == 0{
        return board_utils::evaluate(&board);
    }

    for a in possible_moves{
        v = general_utils::minimum(v,max_player(board_utils::do_move(&mut board.clone(), a),alpha.clone(),beta.clone(),depth-1,!white_turn).await);
        if v <= alpha as f64{
            return v
        }
        beta = general_utils::minimum(beta, v);
    }
    return v
}