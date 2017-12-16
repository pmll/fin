
use piston_window::*;
use common;

const BOMB_WIDTH: f64 = 3.0;
const BOMB_HEIGHT: f64 = 15.0;
const BOMB_SPEED: f64 = 10.0;
const MAX_BOMBS: usize = 6;

#[derive(Copy, Clone)]
struct Bomb {
    x: f64,
    y: f64,
    in_flight: bool,
}

pub struct Bombs {
    bomb: [Bomb; MAX_BOMBS],
    bomb_image: G2dTexture,
}

impl Bombs {
    // for now just get bombs on screen
    pub fn new(window: &mut PistonWindow) -> Bombs {
        Bombs {bomb: [Bomb {x: 0.0, y: 0.0, in_flight: false}; MAX_BOMBS],
               bomb_image: common::win_image(window, "bomb.png")}
    }

    pub fn reset(&mut self) {
        for i in 0..MAX_BOMBS {
            self.bomb[i].in_flight = false;
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for i in 0..6 {
            if self.bomb[i].in_flight {
                image(&self.bomb_image, c.transform.trans(self.bomb[i].x, self.bomb[i].y), g);
            }
        }
    }
}

