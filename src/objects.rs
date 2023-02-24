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
    //pad_list: Vec<Rect>,
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

        Ball {
            circle: Circle {
                x: pos.x,
                y: pos.y,
                r: BALL_RADIUS,
            },
            color: LIGHTGRAY,
            speed: vec2(x, y),
            //pad_list: pad_list
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.color);
    }

    pub fn update(&mut self, pad_list: Vec<Rect>) {
        self.circle.x += self.speed.x;
        self.circle.y += self.speed.y;

        if self.circle.x < 0. || self.circle.x > screen_width() {
            self.speed.x *= -1.;
        }

        if self.circle.y < 0. || self.circle.y > screen_height() {
            self.speed.y *= -1.;
        }

        for pad in pad_list.iter() {
            let new_vec = self.resolve_collision(pad);

            self.speed.x = new_vec.x;
            self.speed.y = new_vec.y;
        }
    }

    fn resolve_collision(&self, pad: &Rect) -> Vec2{
        let mut dx = self.speed.x;
        let mut dy = self.speed.y;

        if self.circle.overlaps_rect(pad) {
            if self.circle.x + self.circle.r >= pad.x ||
            self.circle.x <= pad.x + pad.w {
                dx *= -1.;
            }

            /*if self.circle.y + self.circle.r >= pad.y ||
            self.circle.y <= pad.y + pad.h {
                dy *= -1.;
            }*/
        }

        let new_vec = vec2(dx, dy);

        new_vec
    }
}
