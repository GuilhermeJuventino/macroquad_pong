use macroquad::prelude::*;

use objects::{Pad, PadType, Ball};

mod objects;


#[macroquad::main("Macroquad Pong")]
async fn main() {
    // creating game objects
    let mut player = Pad::new(vec2(40., screen_height() / 2.5), PadType::Player);
    let mut enemy = Pad::new(vec2(screen_width() - 10., screen_height() / 2.5), PadType::Enemy);
    let mut ball = Ball::new(vec2(screen_width() / 2., screen_height() / 2.));

    // main loop
    loop {
        clear_background(BLACK);

        // updating game objects
        player.update();
        enemy.update();
        ball.update(&player.rect, &enemy.rect);

        // drawing game objects
        draw_line(screen_width() / 2., 0., screen_width() / 2., screen_height(), 3., WHITE);

        player.draw();
        enemy.draw();
        ball.draw();

        next_frame().await;
    }
}
