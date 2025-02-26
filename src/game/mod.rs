use piston_window::*;
use piston_window::types::Color;

use rand::{ thread_rng, Rng };

use super::snake::{ Direction, Snake };
use super::draw::{ draw_block, draw_rectangle };

const FOOD_COLOR: Color = [255.0, 51.0, 149.0, 0.84];
const BORDER_COLOR: Color = [255.0, 0.0, 42.0, 0.61];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.5;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }
}
