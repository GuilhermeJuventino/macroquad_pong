use macroquad::prelude::*;

pub enum PadType {
    Player,
    Enemy,
}


// player object
pub struct Pad {
    rect: Rect,
    color: Color,
    pad_type: PadType
}

impl Pad {
    // function that creates a new pad object
    pub fn new(pos: Vec2, pad_type: PadType) -> Self {
        let w = 30.;
        let h = 100.;

        Pad {
            rect: Rect {
                x: pos.x - w,
                y: pos.y,
                w: w,
                h: h
            },
            color: WHITE,
            pad_type: pad_type
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
                
            },
            PadType::Enemy => {}
        }
    }

    // function that draws the pad to the screen at it's current position
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
    }
}