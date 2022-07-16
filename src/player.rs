use std::collections::LinkedList;

use crate::state::{SQUARES};

use macroquad::prelude::*;

type Point = (i16, i16);

#[derive(PartialEq)]
pub enum Direction {
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

pub struct Player {
    pub position: Point,
    pub direction: Direction,
    pub speed: f64,
    pub body: LinkedList<Point>,
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

    pub fn r#move(&mut self){
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