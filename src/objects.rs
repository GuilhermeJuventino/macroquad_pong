use ::rand::prelude::*;
use macroquad::prelude::*;

use crate::constants::*;

pub enum PadType {
    Player,
    Enemy,
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
    pub fn update(&mut self) {
        // check if the pad belongs to the player or the enemy
        match self.pad_type {
            PadType::Player => {
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
            PadType::Enemy => {}
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
}

// ball object
pub struct Ball {
    circle: Circle,
    color: Color,
    speed: Vec2,

    // vector with the position of player and enemy pads
    pad_list: Vec<Rect>,
}

impl Ball {
    pub fn new(pos: Vec2, pad_list: Vec<Rect>) -> Self {
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

        Ball {
            circle: Circle {
                x: pos.x,
                y: pos.y,
                r: BALL_RADIUS,
            },
            color: LIGHTGRAY,
            speed: vec2(x, y),
            pad_list: pad_list
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.color);
    }

    pub fn update(&mut self) {
        self.circle.x += self.speed.x;
        self.circle.y += self.speed.y;

        if self.circle.x < 0. || self.circle.x > screen_width() {
            self.speed.x *= -1.;
        }

        if self.circle.y < 0. || self.circle.y > screen_height() {
            self.speed.y *= -1.;
        }

        for pad in self.pad_list.iter() {
            /*if self.detect_collision("horizontal", pad) {
                self.speed.x *= -1.;
            }

            if self.detect_collision("vertical", pad) {
                self.speed.y *= -1.;
            }*/

            if self.detect_collision("horizontal", pad) {
                println!("True");
            }
        }
    }

    pub fn detect_collision(&self, aim: &str, pad: &Rect) -> bool {
        let collision: bool;

        let mut test_x = self.circle.x;
        let mut test_y = self.circle.y;

        let dist_x: f32;
        let dist_y: f32;

        if self.circle.x < pad.x {
            test_x = pad.x;
        } else if self.circle.x > pad.x + pad.w {
            test_x = pad.x + pad.w;
        }

        if self.circle.y < pad.y {
            test_y = pad.y;
        } else if self.circle.y > pad.y + pad.h {
            test_y = pad.y + pad.h;
        }

        dist_x = self.circle.x - test_x;
        dist_y = self.circle.y - test_y;

        let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

        if distance <= self.circle.r {
            collision = true;
        } else {
            collision = false;
        }

            /*if aim == "horizontal" {
                if self.circle.x < pad.x ||
                self.circle.x > pad.x + pad.w {
                    collision = true;
                } else {
                    collision = false;
                }
            }

            if aim == "vertical" {
                if self.circle.y < pad.y ||
                self.circle.y > pad.y + pad.h {
                    collision = true;
                }
            }*/

        collision
    }
}
