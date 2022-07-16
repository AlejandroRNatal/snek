use macroquad::prelude::*;

pub mod player;
pub mod state;

use state::*;


#[macroquad::main("Snekimus Maximus")]
async fn main() {
    let mut game_state = GameState::new();
    
    loop {
        if game_state.state != State::GameOver {

            game_state.listen_keyboard();

            if get_time() - game_state.time > game_state.player.speed {
                game_state.time = get_time();
                game_state.player.r#move();
                
                if game_state.player.intersects(&game_state.fruit) {
                    game_state.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    game_state.score += 100;
                    game_state.player.speed *= 0.9;
                }
                else {
                    game_state.player.body.pop_back();
                }
                
                if game_state.player_out_of_bounds()
                {
                    game_state.state = State::GameOver;    
                }

                for (x,y) in &game_state.player.body {
                    if game_state.player.intersects(&(*x,*y)) {    
                        game_state.state = State::GameOver;
                    }
                }
            } 
        
            // Draw logic here
            game_state.draw();
        }

        else {
            game_state.draw_game_over();

            if is_key_down(KeyCode::Enter) {
                game_state.reset();
            }
            else if is_key_down(KeyCode::Escape){
                break;
            }
        }
        next_frame().await
    }
}
