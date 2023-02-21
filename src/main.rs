use macroquad::prelude::*;

use objects::{Pad, PadType};

mod objects;


#[macroquad::main("Macroquad Pong")]
async fn main() {
    // creating game objects
    let mut player = Pad::new(vec2(40., screen_height() / 2.5), PadType::Player);
    let mut enemy = Pad::new(vec2(screen_width() - 10., screen_height() / 2.5), PadType::Enemy);

    // main loop
    loop {
        clear_background(BLACK);

        // updating game objects
        player.update();
        enemy.update();

        // drawing game objects
        player.draw();
        enemy.draw();

        next_frame().await;
    }
}
