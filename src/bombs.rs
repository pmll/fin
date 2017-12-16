
use piston_window::*;
use common;

const BOMB_WIDTH: f64 = 3.0;
const BOMB_HEIGHT: f64 = 15.0;
const BOMB_SPEED: f64 = 5.0;
const MAX_BOMBS: usize = 6;

#[derive(Copy, Clone)]
struct Bomb {
    x: f64,
    y: f64,
    in_flight: bool,
}

impl Bomb {
    fn update(&mut self) {
        if self.in_flight {
            self.y += BOMB_SPEED;
            self.in_flight = self.y < common::SCREEN_HEIGHT;
        }
    }
}

pub struct Bombs {
    bomb: [Bomb; MAX_BOMBS],
    bomb_image: G2dTexture,
}

impl Bombs {
    pub fn new(window: &mut PistonWindow) -> Bombs {
        Bombs {bomb: [Bomb {x: 0.0, y: 0.0, in_flight: false}; MAX_BOMBS],
               bomb_image: common::win_image(window, "bomb.png")}
    }

    pub fn reset(&mut self) {
        for i in 0..MAX_BOMBS {
            self.bomb[i].in_flight = false;
        }
    }

    pub fn release(&mut self, x: f64, y: f64) -> bool {
        // this type of thing is undoubtedly not very idiomatic rust,
        // need to find a better way of doing this...
        for i in 0..MAX_BOMBS {
            if ! self.bomb[i].in_flight {
                self.bomb[i].x = x - BOMB_WIDTH / 2.0;
                self.bomb[i].y = y;
                self.bomb[i].in_flight = true;
                return true;
            }
        }
        return false;
    }

    pub fn update(&mut self) {
        for i in 0..MAX_BOMBS {
            self.bomb[i].update();
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

