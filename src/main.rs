use piston_window::{
    clear,
    types::Color,
    Button,
    PistonWindow,
    PressEvent,
    UpdateEvent,
    WindowSettings,
};

extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

const BACKGROUND_COLOR: Color = [0.7, 0.7, 0.7, 0.5];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [
        draw::to_coord_u32(width),
        draw::to_coord_u32(height),
    ])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACKGROUND_COLOR, graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
