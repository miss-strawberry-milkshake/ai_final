use macroquad::prelude::*;
const SQUARES: i16 = 8;
const WHITE_PEICE: i16 = 1;
const RED_PEICE: i16 = -1;
const WHITE_KING: i16 = 2;
const RED_KING: i16 = -2;

const DARK_SQUARE: Color = Color{r: 0.25, g: 0.15, b: 0.1, a: 1.};
const LIGHT_SQUARE: Color = Color{r: 0.9, g: 0.75, b: 0.6, a: 1.};


pub fn draw_board(){

    let game_size: f32 = screen_width().min(screen_height());
    let cube_width: f32 = (game_size)/SQUARES as f32;

    clear_background(DARKGRAY);

    for x in 0..SQUARES {
        for y in 0..SQUARES{
    
            let mut square_color: Color = DARK_SQUARE;
    
            if (y % 2 == 0 && x % 2 == 0) || (y % 2 == 1 && x % 2 == 1){
                square_color = LIGHT_SQUARE;
            }
            draw_rectangle(
                cube_width * x as f32,
                cube_width * y as f32, 
                cube_width,
                cube_width,
                square_color);
        }
    }
}

pub async fn draw_peices(board: [[i16; 8]; 8]){

    let game_size: f32 = screen_width().min(screen_height());
    let cube_width: f32 = (game_size)/SQUARES as f32;

    for x in 0..SQUARES{
        for y in 0..SQUARES{
            if board[x as usize][y as usize] == WHITE_PEICE{


                draw_circle((cube_width * x as f32) + (cube_width/2.), (cube_width * y as f32) + (cube_width/2.), cube_width/3., WHITE);

            } else if board[x as usize][y as usize] == RED_PEICE{
                
                draw_circle((cube_width * x as f32) + (cube_width/2.), (cube_width * y as f32) + (cube_width/2.), cube_width/3., RED);

            } else if board[x as usize][y as usize] == WHITE_KING {

                draw_circle((cube_width * x as f32) + (cube_width/2.), (cube_width * y as f32) + (cube_width/2.), cube_width/3., LIGHTGRAY);
                
            } else if board[x as usize][y as usize] == RED_KING {

                draw_circle((cube_width * x as f32) + (cube_width/2.), (cube_width * y as f32) + (cube_width/2.), cube_width/3., MAROON);
                
            }
        }
    }

    next_frame().await;
}