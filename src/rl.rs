use std::fs::File;
use std::io::Write;
use std::io::Read;
use rand::prelude::*;

use crate::board_utils::do_move;
use crate::board_utils::evaluate;
use crate::board_utils::generate_moves;

const STEP: f32 = 0.0000001;
const SQUARES: usize = 8;


pub fn read_weight_file(weight_vector: &mut [f32; 10], white: bool){

    let mut read_string = String::new();

    let mut f:File;
    if white{
        f = File::open("weight_vector_storage_white.txt").expect("Unable to open file");
    } else{
        f = File::open("weight_vector_storage_red.txt").expect("Unable to open file");
    }
    f.read_to_string(&mut read_string).expect("Unable to read string");

    let mut vect_count: usize = 0;
    let mut cur_string = String::new();
    for i in read_string.chars(){

        if !(i.eq(&' ')){
            cur_string.push(i);
        } else{
            weight_vector[vect_count] = cur_string.trim().parse::<f32>().unwrap();
            vect_count += 1;
            cur_string = "".to_string();
        }
    }
}

pub fn write_weight_file(weight_vector: [f32; 10], white: bool){
    
    let mut write_string = String::new();

    for i in weight_vector{
        write_string.push_str(&i.to_string());
        write_string.push_str(" ");
    }
    let mut f: File;
    if white{
        f = File::create("weight_vector_storage_white.txt").expect("Unable to create file");
    } else{
        f = File::create("weight_vector_storage_red.txt").expect("Unable to create file");
    }
    f.write_all(write_string.as_bytes()).expect("Unable to write data");
}

pub fn feature_gen(board: [[i16; 8]; 8]) -> [f32; 10]{

    let mut num_white_peice: f32 = 0.;
    let mut num_red_peice: f32 = 0.;
    let mut num_white_king: f32 = 0.;
    let mut num_red_king: f32 = 0.;
    let mut num_white_backline: f32 = 0.;
    let mut num_red_backline: f32 = 0.;
    let mut num_white_adjacentne: f32 = 0.;
    let mut num_red_adjacentne:f32 = 0.;
    let mut num_white_center:f32 = 0.;
    let mut num_red_center:f32 = 0.;

    for x in 0..SQUARES{
        for y in 0..SQUARES{
            if board[x][y] == 1{
                num_white_peice += 1.;
            }
            else if board[x][y] == 2{
                num_white_king += 1.;
            }
            else if board[x][y] == -1{
                num_red_peice += 1.;
            }
            else if board[x][y] == -2{
                num_red_king += 1.;
            }
            if y == 0 && (board[x][y] == 1 || board[x][y] == 2){
                num_white_backline += 1.;
            }
            if y == 7 && (board[x][y] == -1 || board[x][y] == -2){
                num_red_backline += 1.;
            }
            if (board[x][y] == 1 || board[x][y] == 2) && 
            ((x+1 < SQUARES && y+1 < SQUARES && (board[x+1][y+1] == 1 || board[x+1][y+1] == 2)) || 
            (x as i32 -1 >= 0 && y+1 < SQUARES && (board[x-1][y+1] == 1 || board[x-1][y+1] == 2)) || 
            (x+1 < SQUARES && y as i32-1 >= 0 && (board[x+1][y-1] == 1 || board[x+1][y-1] == 2)) || 
            (x as i32-1 >= 0 && y as i32-1 >= 0 && (board[x-1][y-1] == 1 || board[x-1][y-1] == 2))){
                num_white_adjacentne += 1.;
            }
            if (board[x][y] == -1 || board[x][y] == -2) && 
            ((x+1 < SQUARES && y+1 < SQUARES && (board[x+1][y+1] == -1 || board[x+1][y+1] == -2)) || 
            (x as i32 -1 >= 0 && y+1 < SQUARES && (board[x-1][y+1] == -1 || board[x-1][y+1] == -2)) || 
            (x+1 < SQUARES && y as i32-1 >= 0 && (board[x+1][y-1] == -1 || board[x+1][y-1] == -2)) || 
            (x as i32-1 >= 0 && y as i32-1 >= 0 && (board[x-1][y-1] == -1 || board[x-1][y-1] == -2))){
                num_red_adjacentne += 1.;
            }
            if (x > 1 && x < 6 && y > 1 && y < 6) && (board[x][y] == 1 || board[x][y] == 2){
                num_white_center += 1.;
            }
            if (x > 1 && x < 6 && y > 1 && y < 6) && (board[x][y] == -1 || board[x][y] == -2){
                num_red_center += 1.;
            }
        }
    }
    let mut feature_vector:[f32; 10] = [0.; 10];
    feature_vector[0] = num_white_peice;
    feature_vector[1] = num_red_peice;
    feature_vector[2] = num_white_king;
    feature_vector[3] = num_red_king;
    feature_vector[4] = num_white_backline;
    feature_vector[5] = num_red_backline;
    feature_vector[6] = num_white_adjacentne;
    feature_vector[7] = num_red_adjacentne;
    feature_vector[8] = num_white_center;
    feature_vector[9] = num_red_center;

    return feature_vector;
}

//Q(s,a) = Q(s,a) + step[r + Q(s', a') - Q(s,a)]

pub fn rl_learner(board: &mut [[i16; 8]; 8], weight_vector: &mut [f32; 10], white_player: bool, epsilon: f32) {

    let mut best_move_value: f32 = 0.;

    let pos_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = generate_moves(board, white_player);
    let pos_moves_len: i32 = pos_moves.len() as i32;
    let mut best_move: (i16,i16,i16,i16,i16,i16,i16) = pos_moves[0];


    //should we take the best move?
    let mut rng = rand::thread_rng();
    let explore: f32 = rng.gen();
    if explore > epsilon{
        let a: i32 = i32::abs(rng.gen::<i32>() % pos_moves_len as i32);
        best_move = pos_moves[a as usize];
    } else {
    //find the best move
    for a in &pos_moves{

        let current_move_evaluated = do_move(&mut board.clone(), *a).clone();
        let current_move_feature = feature_gen(current_move_evaluated);
        let mut total_current_move: f32 = 0.;

        for w in 0..weight_vector.len(){
            total_current_move += current_move_feature[w] as f32 * weight_vector[w] as f32;
        }

        if best_move_value < total_current_move{
            best_move_value = total_current_move;
            best_move = *a;
        }
    }
    }
    let current_state = feature_gen(*board);
    let new_board = do_move(board, best_move);
    let move_we_choose = feature_gen(new_board);

    let final_value = evaluate(&new_board);
    let mut reward: f32 = 0.;
    if final_value == 100.{
        if white_player{
            reward = 10.;
        } else {
            reward = -50.;
        }
    } else if final_value == -100.{
        if white_player{
            reward = -50.;
        } else {
            reward = 10.;
        }
    }

    for w in 0..weight_vector.len(){
        //this is the actual equation
        weight_vector[w] = weight_vector[w] + (STEP * (reward + (move_we_choose[w] * weight_vector[w]) - (current_state[w] * weight_vector[w])));
        if weight_vector[w] > 30.{weight_vector[w] = 30.}
        if weight_vector[w] < -30.{weight_vector[w] = -30.}
    }

}