use ::rand::prelude::*;
use macroquad::prelude::*;

use crate::constants::*;
use crate::utils::display_text;

pub enum PadType {
    Player,
    Enemy,
}

pub enum BallState {
    Active,
    Inactive,
}

// pad object
pub struct Pad {
    pub rect: Rect,
    color: Color,
    pad_type: PadType,
}

impl Pad {
    // function that creates a new pad object
    pub fn new(pos: Vec2, pad_type: PadType) -> Self {
        let w = PAD_SIZE.0;
        let h = PAD_SIZE.1;

        Pad {
            rect: Rect {
                x: pos.x - w,
                y: pos.y,
                w: w,
                h: h,
            },
            color: WHITE,
            pad_type: pad_type,
        }
    }

    // function that updates the pad position and or other related information related to the player
    pub fn update(&mut self, ball: &Circle, state: &BallState) {
        // check if the pad belongs to the player or the enemy
        match self.pad_type {
            PadType::Player => {
                match state {
                    BallState::Active => {
                        // player controls
                        let move_y = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
                            (true, false) => -5.,
                            (false, true) => 5.,
                            _ => 0.,
                        };

                        // updating player position
                        self.rect.y += move_y;

                        // preventing player pad from leaving the screen
                        if self.rect.y < 0. {
                            self.rect.y = 0.;
                        } else if self.rect.y > screen_height() - self.rect.h {
                            self.rect.y = screen_height() - self.rect.h;
                        }
                    }
                    _ => (),
                }
            }
            PadType::Enemy => {
                match state {
                    BallState::Active => {
                        let move_y = self.follow_ball(ball);

                        self.rect.y += move_y;

                        // preventing player pad from leaving the screen
                        if self.rect.y < 0. {
                            self.rect.y = 0.;
                        } else if self.rect.y > screen_height() - self.rect.h {
                            self.rect.y = screen_height() - self.rect.h;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // function that draws the pad to the screen at it's current position
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );
    }

    // functions that makes the enemy pad try to follow the ball
    pub fn follow_ball(&mut self, ball: &Circle) -> f32 {
        let mut move_y: f32 = 0.;

        if self.rect.top() < ball.y {
            move_y = 4.;
        }

        if self.rect.bottom() > ball.y {
            move_y = -4.;
        }

        move_y
    }

    // function that reset the pad's position
    pub fn reset_position(&mut self) {
        self.rect.y = screen_height() / 2.5;
    }
}

// ball object
pub struct Ball {
    pub circle: Circle,
    color: Color,
    speed: Vec2,
    pub state: BallState,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        // randomizing ball's initial velocity
        let mut rng = thread_rng();
        let mut x = rng.gen_range(-BALL_SPEED..BALL_SPEED);
        let mut y = rng.gen_range(-BALL_SPEED..BALL_SPEED);

        // making sure the ball moves at the same speed in either direction
        if x > 0. {
            x = BALL_SPEED;
        } else if x < 0. {
            x = -BALL_SPEED;
        }

        if y > 0. {
            y = BALL_SPEED;
        } else if y < 0. {
            y = -BALL_SPEED;
        }

        // creating and returning the instance of the Ball struct
        Ball {
            circle: Circle {
                x: pos.x,
                y: pos.y,
                r: BALL_RADIUS,
            },
            color: LIGHTGRAY,
            speed: vec2(x, y),
            state: BallState::Inactive,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.color);
    }

    pub fn update(&mut self, pad_list: Vec<Rect>, score: &mut Score) {
        match self.state {
            BallState::Active => {
                // updates the ball's position
                self.circle.x += self.speed.x;
                self.circle.y += self.speed.y;

                // reseting the ball's position after it leaves the screen from either the left or right side
                /*if self.circle.x < 0. || self.circle.x > screen_width() {
                    self.reset_position();
                }*/

                if self.circle.x < 0. {
                    score.enemy += 1;
                    self.reset_position()
                } else if self.circle.x > screen_width() {
                    score.player += 1;
                    self.reset_position();
                }

                // preventing the ball from going above or below the screen
                if self.circle.y < 0. || self.circle.y > screen_height() {
                    self.speed.y *= -1.;
                }

                for pad in pad_list.iter() {
                    // checks for collision and gets the updated x/y velocities from the resolve_collision function
                    let new_vec = self.resolve_collision(pad);

                    // updates self.speed.x/y
                    self.speed.x = new_vec.x;
                    self.speed.y = new_vec.y;
                }
            }
            BallState::Inactive => {
                if is_key_pressed(KeyCode::Space) {
                    self.activate_ball();
                }
            }
        }
    }

    // function that resolves ball vs pad collisions
    fn resolve_collision(&self, pad: &Rect) -> Vec2 {
        // temporary x/y values
        let mut dx = self.speed.x;
        let mut dy = self.speed.y;

        // check if the ball is colliding with a pad
        if self.circle.overlaps_rect(pad) {
            // checks if the ball speed is less than 0.
            if self.speed.x < 0. {
                // checks where the ball hit the pad
                if self.circle.y >= pad.y && self.circle.y <= pad.y + pad.h {
                    if self.circle.x - self.circle.r <= pad.x + pad.w {
                        // update temporary x velocity
                        dx *= -1.;

                        // calculating the new temporary y velocity
                        let middle_y = pad.y + pad.h / 2.;
                        let displacement = middle_y - self.circle.y;
                        let reduction_factor = (pad.h / 2.) / BALL_SPEED;

                        // updates temporary y velocity
                        dy = (displacement / reduction_factor) * -1.;
                    }
                }

            // checks if the ball speed is greater than 0.
            } else if self.speed.x > 0. {
                // checks where the ball hit the pad
                if self.circle.y >= pad.y && self.circle.y <= pad.y + pad.h {
                    if self.circle.x + self.circle.r >= pad.x {
                        // updates temporary x velocity
                        dx *= -1.;

                        // calculating the new temporary y velocity
                        let middle_y = pad.y + pad.h / 2.;
                        let displacement = middle_y - self.circle.y;
                        let reduction_factor = (pad.h / 2.) / BALL_SPEED;

                        // updates temporary y velocity
                        dy = (displacement / reduction_factor) * -1.;
                    }
                }
            }
        }

        // creates a new vector with the temporary x and y velocity and returns it
        let new_vec = vec2(dx, dy);

        new_vec
    }

    // function that resets the ball's position and state
    pub fn reset_position(&mut self) {
        // reseting ball's position and state
        self.state = BallState::Inactive;
        self.circle.x = screen_width() / 2.;
        self.circle.y = screen_height() / 2.;

        // reseting ball's initial velocity
        let mut rng = thread_rng();

        let mut x = rng.gen_range(-BALL_SPEED..BALL_SPEED);
        let mut y = rng.gen_range(-BALL_SPEED..BALL_SPEED);

        if x > 0. {
            x = BALL_SPEED;
        } else if x < 0. {
            x = -BALL_SPEED
        }

        if y > 0. {
            y = BALL_SPEED;
        } else if y < 0. {
            y = -BALL_SPEED;
        }

        self.speed.x = x;
        self.speed.y = y;
    }

    // function that changes the ball state from inactive to active
    fn activate_ball(&mut self) {
        self.state = BallState::Active;
    }
}

// a struct that contains the game score
pub struct Score {
    player: u32,
    enemy: u32,
}

impl Score {
    pub fn new() -> Self {
        Score {
            player: 0,
            enemy: 0,
        }
    }

    pub fn reset_score(&mut self) {
        self.player = 0;
        self.enemy = 0;
    }

    pub fn display_score(&self, font: &Font) {
        let player_score_text = format!("{}", self.player);
        let enemy_score_text = format!("{}", self.enemy);

        display_text(
            player_score_text.as_str(),
            (screen_width() / 2.) - 40.,
            screen_height() / 2.,
            font,
            40,
            RED
        );

        display_text(
            enemy_score_text.as_str(),
            (screen_width() / 2.) + 40.,
            screen_height() / 2.,
            font,
            40,
            RED
        );
    }
}
