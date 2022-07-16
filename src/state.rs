
use crate::player::{Player, Direction};
use macroquad::prelude::*;
use std::ops::Not;

type Point = (i16, i16);

pub const SQUARES: i16 = 16;

#[derive(PartialEq)]
pub enum State {
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

pub struct GameState {
    pub score: u32,
    pub state: State,
    pub player: Player,
    pub fruit: Point,
    pub time: f64,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            state: State::Running,
            player: Player::new(),
            fruit: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
            time: get_time(),
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.state = State::Running;
        self.player.reset();
        self.time = get_time();
        self.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
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

    pub fn draw(&self){

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
            Self::offset_x() + self.fruit.0 as f32 * Self::square_size(),
            Self::offset_y() + self.fruit.1 as f32 * Self::square_size(),
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

    pub fn listen_keyboard(&mut self){
        if is_key_down(KeyCode::Right) && self.player.direction != Direction::Left {
            self.player.direction = Direction::Right;    
        }
        
        if is_key_down(KeyCode::Left) && self.player.direction != Direction::Right {
            self.player.direction = Direction::Left;    
        }

        if is_key_down(KeyCode::Up) && self.player.direction != Direction::Down {
            self.player.direction = Direction::Up;    
        }

        if is_key_down(KeyCode::Down) && self.player.direction != Direction::Up {
            self.player.direction = Direction::Down;    
        }
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

    pub fn player_out_of_bounds(&self) -> bool{
        self.player.position.0 < 0 ||
        self.player.position.1 < 0 ||
        self.player.position.0 >= SQUARES ||
        self.player.position.1 >= SQUARES
    }
}