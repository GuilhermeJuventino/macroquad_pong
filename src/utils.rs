use macroquad::prelude::*;

use crate::objects::*;

// function for displaying text to the screen
pub fn display_text(text: &str, x: f32, y: f32, font: &Font) {
    let text_params = TextParams {
        font: *font,
        font_size: 40,
        font_scale: 1.,
        color: RED,
        ..Default::default()
    };

    let text_dim = measure_text(text, Some(*font), 40, 1.);

    draw_text_ex(
        text,
        x - text_dim.width / 2.,
        y + text_dim.height / 2.,
        text_params,
    );
}

// function for reseting the game
pub fn reset_game(player: &mut Pad, enemy: &mut Pad, ball: &mut Ball, score: &mut Score) {
    score.reset_score();
    player.reset_position();
    enemy.reset_position();
    ball.reset_position();
}
