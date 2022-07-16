use macroquad::prelude::*;
use std::collections::LinkedList;
use std::ops::Not;

type Point = (i16, i16);

const SQUARES: i16 = 16;


#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_direction(dir: &Direction) -> Point {
        match dir {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            _ => (0, 0),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Running,
    GameOver,
    Pause,
    Rendering,
}

impl Not for State
{
    type Output = Self;

    fn not(self) -> Self {
        match self{
            State::Running => State::GameOver,
            State::GameOver => State::Running,
            State::Pause => State::Running,
            State::Rendering => State::Pause,
        }
    }
}

struct Player {
    position: Point,
    direction: Direction,
    speed: f64,
    body: LinkedList<Point>,
}

impl Player {
    pub fn new() -> Self {
        Player{
            position: (0, 0),
            direction: Direction::Down,
            speed: 0.3,
            body: LinkedList::new(),
        }
    }

    pub fn reset(&mut self){
        self.position = (0,0);
        self.direction = Direction::Down;
        self.speed = 0.3;
        self.body = LinkedList::new();
    }

    pub fn intersects(&self, other: &Point) -> bool {
        self.position == *other
    }

    fn r#move(&mut self){
        self.body.push_front(self.position);
        self.position = (self.position.0 + Direction::from_direction(&self.direction).0,
                         self.position.1 + Direction::from_direction(&self.direction).1);
    }

    pub fn draw(&self) {
        let game_size = screen_width().min(screen_height());
        let offset_x  = (screen_width() - game_size)/2.0 + 10.0;
        let offset_y = (screen_height()- game_size)/2.0 + 10.0;

        let square_size = (screen_height() - offset_y * 2.0)/ SQUARES as f32;

        // Draw Head,
        draw_rectangle(
            offset_x + self.position.0.clone() as f32 * square_size,
            offset_y + self.position.1.clone() as f32 * square_size,
            square_size,
            square_size,
            GREEN,
        );

        // Draw Rest of body
        for (x,y) in &self.body {
            draw_rectangle(
                offset_x + *x as f32 * square_size,
                offset_y + *y as f32 * square_size,
                square_size,
                square_size,
                LIME
            );
        }
    }

}

struct GameState {
    score: u32,
    state: State,
    player: Player,
    time: f64,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            state: State::Running,
            player: Player::new(),
            time: get_time(),
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.state = State::Running;
        self.player.reset();
        self.time = get_time(); 
    }

    pub fn offset_y() -> f32 {
        (screen_height() - Self::game_size()) / 2.0 + 10.0
    }

    pub fn offset_x() -> f32 {
        (screen_width() - Self::game_size()) / 2.0 + 10.
    }

    pub fn square_size() -> f32 {
        (screen_height() - Self::offset_y() * 2.0) / SQUARES as f32
    }

    pub fn game_size() -> f32 {
        screen_width().min(screen_height()) 
    }

    pub fn draw(&self, fruit: &Point){

        clear_background(LIGHTGRAY);

        draw_rectangle(Self::offset_x(), Self::offset_y(), Self::game_size() - 20.0, Self::game_size()-20.0, WHITE);

        for i in 1..SQUARES {
            draw_line(
                Self::offset_x(),
                Self::offset_y() + Self::square_size() * i as f32,
                screen_width() - Self::offset_x(),
                Self::offset_y() + Self::square_size() * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            draw_line(
                Self::offset_x() + Self::square_size() * i as f32,
                Self::offset_y(),
                Self::offset_x() + Self::square_size() * i as f32,
                screen_height() - Self::offset_y(),
                2.,
                LIGHTGRAY,
            );
        }

        self.player.draw();
        draw_rectangle(
            Self::offset_x() + fruit.0 as f32 * Self::square_size(),
            Self::offset_y() + fruit.1 as f32 * Self::square_size(),
            Self::square_size(),
            Self::square_size(),
            GOLD,
        );

        draw_text(
            format!("SCORE: {}", self.score).as_str(),
            10.,
            10.,
            20.,
            DARKGRAY,
        );
    }

    pub fn draw_game_over(&self) {
        clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
    }

    fn player_out_of_bounds(&self) -> bool{
        self.player.position.0 < 0 ||
        self.player.position.1 < 0 ||
        self.player.position.0 >= SQUARES ||
        self.player.position.1 >= SQUARES
    }
}

#[macroquad::main("Snekimus Maximus")]
async fn main() {
    let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    let mut game_state = GameState::new();
    
    loop {
        if game_state.state != State::GameOver{

            if is_key_down(KeyCode::Right) && game_state.player.direction != Direction::Left {
                game_state.player.direction = Direction::Right;    
            }
            
            if is_key_down(KeyCode::Left) && game_state.player.direction != Direction::Right {
                game_state.player.direction = Direction::Left;    
            }

            if is_key_down(KeyCode::Up) && game_state.player.direction != Direction::Down {
                game_state.player.direction = Direction::Up;    
            }

            if is_key_down(KeyCode::Down) && game_state.player.direction != Direction::Up {
                game_state.player.direction = Direction::Down;    
            }

            if get_time() - game_state.time > game_state.player.speed {
                game_state.time = get_time();
                game_state.player.r#move();
                
                if game_state.player.intersects(&fruit) {
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
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
            game_state.draw(&fruit);
        }

        else {
            
            game_state.draw_game_over();

            if is_key_down(KeyCode::Enter) {
                fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                game_state.reset();
            }

            else if is_key_down(KeyCode::Escape){
                break;
            }
        }
        next_frame().await
    }
}
