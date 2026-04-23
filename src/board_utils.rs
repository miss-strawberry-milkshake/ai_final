const SQUARES: i16 = 8;

const WHITE_PEICE: i16 = 1;
const RED_PEICE: i16 = -1;
const WHITE_KING: i16 = 2;
const RED_KING: i16 = -2;
const NO_PEICE: i16 = 0;
const NO_JUMP: i16 = 0;
const JUMP: i16 = 1;
const DOUBLE_JUMP: i16 = 2;
// const POS_INF: f64 = std::f64::INFINITY;
// const NEG_INF: f64 = std::f64::NEG_INFINITY;

pub fn generate_board() -> [[i16; 8]; 8]{
    let mut board: [[i16; 8]; 8] = [[0;8];8];
    for x in 0..SQUARES{
        for y in 0..SQUARES{
            if ((x % 2 != 0 && y % 2 == 0) || (x % 2 != 1 && y % 2 == 1)) && (y != 3 && y != 4){

                if y == 0 || y == 1 || y == 2{
                    board[x as usize][y as usize] = WHITE_PEICE;
                }
                else{
                    board[x as usize][y as usize] = RED_PEICE;
                }

            }
        }
    }

    return board.clone();
}

pub fn do_move(board: &mut[[i16; 8]; 8], user_move: (i16,i16,i16,i16,i16,i16,i16)) -> [[i16; 8]; 8]{
    if user_move.4 == DOUBLE_JUMP{
        board[user_move.5 as usize][user_move.6 as usize] = board[user_move.0 as usize][user_move.1 as usize];
        board[((user_move.0+user_move.2)/2) as usize][((user_move.1+user_move.3)/2) as usize] = 0;
        board[((user_move.2+user_move.5)/2) as usize][((user_move.3+user_move.6)/2) as usize] = 0;
        board[user_move.0 as usize][user_move.1 as usize] = 0;
    } else{
    board[user_move.2 as usize][user_move.3 as usize] = board[user_move.0 as usize][user_move.1 as usize];
    board[user_move.0 as usize][user_move.1 as usize] = 0;
    if user_move.4 == JUMP{
        board[((user_move.0+user_move.2)/2) as usize][((user_move.1+user_move.3)/2) as usize] = 0;
    }}

    return *board
}

pub fn generate_moves(board: &mut [[i16; 8]; 8], white_turn: bool) -> Vec<(i16,i16,i16,i16,i16,i16,i16)>{

    let mut possible_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = Vec::new();

    for x in 0..SQUARES{
        if board[x as usize][0] == RED_PEICE{
            board[x as usize][0] = -2;
        }
        if board[x as usize][(SQUARES-1) as usize] == WHITE_PEICE{
            board[x as usize][(SQUARES-1) as usize] = 2;
        }
        for y in 0..SQUARES{
            let mut dj = false;
            if board[x as usize][y as usize] == WHITE_PEICE && white_turn{
                if x+1 >= SQUARES || y+1 >= SQUARES{}
                else if board[x as usize +1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y+1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y+2 >= SQUARES{
                } else if (board[x as usize +1][y as usize +1] == RED_PEICE || board[x as usize +1][y as usize +1] == RED_KING) && board[x as usize +2][y as usize +2] == NO_PEICE{

                    if x+4 >= SQUARES || y+4 >= SQUARES{}
                    else if (board[x as usize +3][y as usize +3] == RED_PEICE || board[x as usize +3][y as usize +3] == RED_KING) && board[x as usize +4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x+4,y+4));
                        dj = true;
                    }else if (board[x as usize +1][y as usize +3] == RED_PEICE || board[x as usize +1][y as usize +3] == RED_KING) && board[x as usize +0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x+2,y+2, JUMP,0,0));
                    }
                }
                if x-1 < 0 || y+1 >= SQUARES{
                } else if board[x as usize -1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y+1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y+2 >= SQUARES{
                } else if (board[x as usize -1][y as usize +1] == RED_PEICE || board[x as usize -1][y as usize +1] == RED_KING) && board[x as usize -2][y as usize +2] == NO_PEICE{

                    if x-4 < 0 || y+4 >= SQUARES{}
                    else if (board[x as usize -3][y as usize +3] == RED_PEICE || board[x as usize -3][y as usize +3] == RED_KING) && board[x as usize -4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x-4,y+4));
                        dj = true;
                    }else if (board[x as usize -1][y as usize +3] == RED_PEICE || board[x as usize -1][y as usize +3] == RED_KING) && board[x as usize -0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj {
                        possible_moves.push((x,y,x-2,y+2, JUMP,0,0));
                    }
                }
            }
            if board[x as usize][y as usize] == RED_PEICE && !white_turn{
                if x-1 < 0 || y-1 < 0{
                } else if board[x as usize -1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y-1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y-2 < 0{
                } else if (board[x as usize -1][y as usize -1] == WHITE_PEICE || board[x as usize -1][y as usize -1] == WHITE_KING) && board[x as usize -2][y as usize -2] == NO_PEICE{
                    
                    if x-4 < 0 || y-4 < 0{}
                    else if (board[x as usize -3][y as usize -3] == WHITE_PEICE || board[x as usize -3][y as usize -3] == WHITE_KING) && board[x as usize -4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x-4,y-4));
                        dj = true;
                    }else if (board[x as usize -1][y as usize -3] == WHITE_PEICE || board[x as usize -3][y as usize -1] == WHITE_KING) && board[x as usize -0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x-2,y-2, JUMP,0,0));
                    }
                }
                if x+1 >= SQUARES || y-1 < 0{
                } else if board[x as usize +1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y-1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y-2 < 0{
                } else if (board[x as usize +1][y as usize -1] == WHITE_PEICE || board[x as usize +1][y as usize -1] == WHITE_KING) && board[x as usize +2][y as usize -2] == NO_PEICE{

                    if x+4 >= SQUARES || y-4 < 0{}
                    else if (board[x as usize +3][y as usize -3] == WHITE_PEICE || board[x as usize +3][y as usize -3] == WHITE_KING) && board[x as usize +4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x+4,y-4));
                        dj = true;
                    }else if (board[x as usize +1][y as usize -3] == WHITE_PEICE || board[x as usize +1][y as usize -3] == WHITE_KING) && board[x as usize +0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x+2,y-2, JUMP,0,0));
                    }
                }
            }
            if board[x as usize][y as usize] == WHITE_KING && white_turn{
                if x+1 >= SQUARES || y+1 >= SQUARES{}
                else if board[x as usize +1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y+1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y+2 >= SQUARES{
                } else if (board[x as usize +1][y as usize +1] == RED_PEICE || board[x as usize +1][y as usize +1] == RED_KING) && board[x as usize +2][y as usize +2] == NO_PEICE{

                    if x+4 >= SQUARES || y+4 >= SQUARES{}
                    else if (board[x as usize +3][y as usize +3] == RED_PEICE || board[x as usize +3][y as usize +3] == RED_KING) && board[x as usize +4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x+4,y+4));
                        dj = true;
                    } else if (board[x as usize +3][y as usize +1] == RED_PEICE || board[x as usize +3][y as usize +1] == RED_KING) && board[x as usize +4][y as usize +0] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x+4,y));
                        dj = true;
                    } else if (board[x as usize +1][y as usize +3] == RED_PEICE || board[x as usize +1][y as usize +3] == RED_KING) && board[x as usize +0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x+2,y+2, JUMP,0,0));
                    }
                }
                if x-1 < 0 || y+1 >= SQUARES{
                } else if board[x as usize -1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y+1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y+2 >= SQUARES{
                } else if (board[x as usize -1][y as usize +1] == RED_PEICE || board[x as usize -1][y as usize +1] == RED_KING) && board[x as usize -2][y as usize +2] == NO_PEICE{

                    if x-4 < 0 || y+4 >= SQUARES{}
                    else if (board[x as usize -3][y as usize +3] == RED_PEICE || board[x as usize -3][y as usize +3] == RED_KING) && board[x as usize -4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x-4,y+4));
                        dj = true;
                    } else if (board[x as usize -3][y as usize +1] == RED_PEICE || board[x as usize -3][y as usize +1] == RED_KING) && board[x as usize -4][y as usize +0] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x-4,y));
                        dj = true;
                    } else if (board[x as usize -1][y as usize +3] == RED_PEICE || board[x as usize -1][y as usize +3] == RED_KING) && board[x as usize -0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj {
                        possible_moves.push((x,y,x-2,y+2, JUMP,0,0));
                    }
                }
                if x-1 < 0 || y-1 < 0{
                } else if board[x as usize -1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y-1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y-2 < 0{
                } else if (board[x as usize -1][y as usize -1] == RED_PEICE || board[x as usize -1][y as usize -1] == RED_KING) && board[x as usize -2][y as usize -2] == NO_PEICE{
                    
                    if x-4 < 0 || y-4 < 0{}
                    else if (board[x as usize -3][y as usize -3] == RED_PEICE || board[x as usize -3][y as usize -3] == RED_KING) && board[x as usize -4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x-4,y-4));
                        dj = true;
                    } else if (board[x as usize -3][y as usize -1] == RED_PEICE || board[x as usize -3][y as usize -1] == RED_KING) && board[x as usize -4][y as usize -0] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x-4,y));
                        dj = true;
                    } else if (board[x as usize -1][y as usize -3] == RED_PEICE || board[x as usize -3][y as usize -1] == RED_KING) && board[x as usize -0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x-2,y-2, JUMP,0,0));
                    }
                }
                if x+1 >= SQUARES || y-1 < 0{
                } else if board[x as usize +1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y-1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y-2 < 0{
                } else if (board[x as usize +1][y as usize -1] == RED_PEICE || board[x as usize +1][y as usize -1] == RED_KING) && board[x as usize +2][y as usize -2] == NO_PEICE{

                    if x+4 >= SQUARES || y-4 < 0{}
                    else if (board[x as usize +3][y as usize -3] == RED_PEICE || board[x as usize +3][y as usize -3] == RED_KING) && board[x as usize +4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x+4,y-4));
                        dj = true;
                    } else if (board[x as usize +3][y as usize -1] == RED_PEICE || board[x as usize +3][y as usize -1] == RED_KING) && board[x as usize +4][y as usize -0] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x+4,y));
                        dj = true;
                    } else if (board[x as usize +1][y as usize -3] == RED_PEICE || board[x as usize +1][y as usize -3] == RED_KING) && board[x as usize +0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    } 
                    if !dj{
                        possible_moves.push((x,y,x+2,y-2, JUMP,0,0));
                    }
                }
            }

            if board[x as usize][y as usize] == RED_KING && !white_turn{
                if x+1 >= SQUARES || y+1 >= SQUARES{}
                else if board[x as usize +1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y+1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y+2 >= SQUARES{
                } else if (board[x as usize +1][y as usize +1] == WHITE_PEICE || board[x as usize +1][y as usize +1] == WHITE_KING) && board[x as usize +2][y as usize +2] == NO_PEICE{

                    if x+4 >= SQUARES || y+4 >= SQUARES{}
                    else if (board[x as usize +3][y as usize +3] == WHITE_PEICE || board[x as usize +3][y as usize +3] == WHITE_KING) && board[x as usize +4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x+4,y+4));
                        dj = true;
                    } else if (board[x as usize +3][y as usize +1] == WHITE_PEICE || board[x as usize +3][y as usize +1] == WHITE_KING) && board[x as usize +4][y as usize +0] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x+4,y));
                        dj = true;
                    } else if (board[x as usize +1][y as usize +3] == WHITE_PEICE || board[x as usize +1][y as usize +3] == WHITE_KING) && board[x as usize +0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x+2,y+2, JUMP,0,0));
                    }
                }
                if x-1 < 0 || y+1 >= SQUARES{
                } else if board[x as usize -1][y as usize +1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y+1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y+2 >= SQUARES{
                } else if (board[x as usize -1][y as usize +1] == WHITE_PEICE || board[x as usize -1][y as usize +1] == WHITE_KING) && board[x as usize -2][y as usize +2] == NO_PEICE{

                    if x-4 < 0 || y+4 >= SQUARES{}
                    else if (board[x as usize -3][y as usize +3] == WHITE_PEICE || board[x as usize -3][y as usize +3] == WHITE_KING) && board[x as usize -4][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x-4,y+4));
                        dj = true;
                    } else if (board[x as usize -3][y as usize +1] == WHITE_PEICE || board[x as usize -3][y as usize +1] == WHITE_KING) && board[x as usize -4][y as usize +0] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x-4,y));
                        dj = true;
                    } else if (board[x as usize -1][y as usize +3] == WHITE_PEICE || board[x as usize -1][y as usize +3] == WHITE_KING) && board[x as usize -0][y as usize +4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y+2, DOUBLE_JUMP,x,y+4));
                        dj = true;
                    }
                    if !dj {
                        possible_moves.push((x,y,x-2,y+2, JUMP,0,0));
                    }
                }
                if x-1 < 0 || y-1 < 0{
                } else if board[x as usize -1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x-1,y-1, NO_JUMP,0,0));

                } else if  x-2 < 0 || y-2 < 0{
                } else if (board[x as usize -1][y as usize -1] == WHITE_PEICE || board[x as usize -1][y as usize -1] == WHITE_KING) && board[x as usize -2][y as usize -2] == NO_PEICE{
                    
                    if x-4 < 0 || y-4 < 0{}
                    else if (board[x as usize -3][y as usize -3] == WHITE_PEICE || board[x as usize -3][y as usize -3] == WHITE_KING) && board[x as usize -4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x-4,y-4));
                        dj = true;
                    } else if (board[x as usize -3][y as usize -1] == WHITE_PEICE || board[x as usize -3][y as usize -1] == WHITE_KING) && board[x as usize -4][y as usize -0] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x-4,y));
                        dj = true;
                    } else if (board[x as usize -1][y as usize -3] == WHITE_PEICE || board[x as usize -3][y as usize -1] == WHITE_KING) && board[x as usize -0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x-2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    }
                    if !dj{
                        possible_moves.push((x,y,x-2,y-2, JUMP,0,0));
                    }
                }
                if x+1 >= SQUARES || y-1 < 0{
                } else if board[x as usize +1][y as usize -1] == NO_PEICE {
                    possible_moves.push((x,y,x+1,y-1, NO_JUMP,0,0));

                } else if  x+2 >= SQUARES || y-2 < 0{
                } else if (board[x as usize +1][y as usize -1] == WHITE_PEICE || board[x as usize +1][y as usize -1] == WHITE_KING) && board[x as usize +2][y as usize -2] == NO_PEICE{

                    if x+4 >= SQUARES || y-4 < 0{}
                    else if (board[x as usize +3][y as usize -3] == WHITE_PEICE || board[x as usize +3][y as usize -3] == WHITE_KING) && board[x as usize +4][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x+4,y-4));
                        dj = true;
                    } else if (board[x as usize +3][y as usize -1] == WHITE_PEICE || board[x as usize +3][y as usize -1] == WHITE_KING) && board[x as usize +4][y as usize -0] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x+4,y));
                        dj = true;
                    } else if (board[x as usize +1][y as usize -3] == WHITE_PEICE || board[x as usize +1][y as usize -3] == WHITE_KING) && board[x as usize +0][y as usize -4] == NO_PEICE{
                        possible_moves.push((x,y,x+2,y-2, DOUBLE_JUMP,x,y-4));
                        dj = true;
                    } 
                    if !dj{
                        possible_moves.push((x,y,x+2,y-2, JUMP,0,0));
                    }
                }
            }
        }
    }

    //remove moves that are not avalible
    jump_logic(&mut possible_moves);

    return possible_moves;
}

fn jump_logic(possible_moves: &mut Vec<(i16,i16,i16,i16,i16,i16,i16)>){
    for jump_check in 0..possible_moves.len(){
        if possible_moves[jump_check].4 != NO_JUMP{
            let mut jump_erase: usize = 0;
            loop{
                if possible_moves[jump_erase].4 == NO_JUMP{
                    possible_moves.swap_remove(jump_erase);
                    jump_erase = 0;
                }else{
                    jump_erase += 1;
                }
                if jump_erase >= possible_moves.len(){
                    break;
                }
            }
        }
        if jump_check >= possible_moves.len()-1{
            break;
        }
    }
}


pub fn evaluate(board: &[[i16; 8]; 8])->f64{
    let white_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = generate_moves(& mut board.clone(), true);
    let red_moves: Vec<(i16,i16,i16,i16,i16,i16,i16)> = generate_moves(& mut board.clone(), false);

    let mut val:f64 = 0.0;
    for x in 0..SQUARES{
        for y in 0..SQUARES{
            val += board[x as usize][y as usize] as f64;
        }
    }

    if white_moves.len() == 0{
        val = -100.;
    }
    if red_moves.len() == 0{
        val = 100.;
    }
    return val;
}