use macroquad::prelude::*;

use constants::*;
use objects::{Ball, Pad, PadType, Score};
use utils::{display_text, reset_game};

mod constants;
mod objects;
mod utils;

// enum for managing game states
enum GameState {
    TitleScreen,
    InGame,
    GameOver,
}

// game configuration
fn window_config() -> Conf {
    Conf {
        window_title: "Macroquad Pong".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // variable for controlling the game state
    let mut game_state = GameState::TitleScreen;

    // loading the game font
    let font = load_ttf_font(FONT).await.expect("Failed to load font");

    // bool variable used to pause the game
    let mut paused = false;

    // creating game objects
    let mut player = Pad::new(vec2(40., screen_height() / 2.5), PadType::Player);

    let mut enemy = Pad::new(
        vec2(screen_width() - 10., screen_height() / 2.5),
        PadType::Enemy,
    );

    let mut ball = Ball::new(vec2(screen_width() / 2., screen_height() / 2.));

    let mut score = Score::new();

    // main loop
    loop {
        clear_background(BLACK);

        // updates the game states
        match game_state {
            GameState::TitleScreen => {
                if is_key_pressed(KeyCode::Enter) {
                    // change the state to in game
                    game_state = GameState::InGame;
                }
            },

            GameState::InGame => {
                // pauses the game whenever the player presses enter
                if is_key_pressed(KeyCode::Enter) {
                    if paused {
                        paused = false;
                    } else {
                        paused = true;
                    }
                }

                if is_key_pressed(KeyCode::S) {
                    // reseting the game
                    reset_game(&mut player, &mut enemy, &mut ball, &mut score);
                    paused = false;
                    
                    // change the state to game over
                    game_state = GameState::GameOver
                }

                // updates the game objects only if the game is not paused
                if !paused {
                    // a list with a copy of both pad's rectangles.
                    let mut pad_list = vec![];

                    // updating game objects
                    player.update(&ball.circle, &ball.state);
                    pad_list.push(player.rect);

                    enemy.update(&ball.circle, &ball.state);
                    pad_list.push(enemy.rect);

                    ball.update(pad_list, &mut score);
                }
            },

            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    // change the state to in game

                    game_state = GameState::InGame;
                }
            }
        }

        // draws the game states
        match game_state {
            GameState::TitleScreen => {
                // displaying title text and game instructions
                let title_screen_text = "Macroquad Pong";
                let instruction_text = "Press Enter to start the game. Move the pad with the arrow keys. Launch the ball with Space";

                display_text(
                    title_screen_text,
                    screen_width() / 2.,
                    screen_height() / 2. - 20.,
                    &font,
                    40,
                    WHITE
                );

                display_text(
                    instruction_text,
                    screen_width() / 2.,
                    screen_height() / 2. + 40.,
                    &font,
                    20,
                    WHITE
                );
            },

            GameState::InGame => {
                // drawing game objects
                draw_line(
                    screen_width() / 2.,
                    0.,
                    screen_width() / 2.,
                    screen_height(),
                    3.,
                    WHITE,
                );

                // drawing the score if the game is not paused
                if !paused {
                    score.display_score(&font);
                }

                player.draw();
                enemy.draw();
                ball.draw();

                // drawing paused text if the game is paused
                if paused {
                    // drawing paused text to the center of the screen
                    let paused_text: &str = "Pause";

                    display_text(
                        paused_text,
                        screen_width() / 2.,
                        screen_height() / 2.,
                        &font,
                        40,
                        RED
                    )
                }
            },

            GameState::GameOver => {
                // displaying game over text
                let game_over_text = "Game Over: Press Enter to play again";

                display_text(
                    game_over_text,
                    screen_width() / 2.,
                    screen_height() / 2.,
                    &font,
                    40,
                    WHITE
                );
            }
        }

        next_frame().await;
    }
}
