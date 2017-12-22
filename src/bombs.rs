
use piston_window::*;
use common;

const BOMB_WIDTH: f64 = 3.0;
const BOMB_HEIGHT: f64 = 15.0;
const BOMB_SPEED: f64 = 4.0;
const MAX_BOMBS: usize = 4;

#[derive(Copy, Clone)]
struct Bomb {
    x: f64,
    y: f64,
    in_flight: bool,
}

impl Bomb {
    fn area(&self) -> common::ScreenObjectArea {
        common::ScreenObjectArea::new(self.x, self.y, BOMB_WIDTH, BOMB_HEIGHT)
    }

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
        for b in &mut self.bomb {
            b.in_flight = false;
        }
    }

    pub fn release(&mut self, x: f64, y: f64) -> bool {
        for b in self.bomb.iter_mut().filter(|b| ! b.in_flight).take(1) {
            b.x = x - BOMB_WIDTH / 2.0;
            b.y = y;
            b.in_flight = true;
            return true;
        }
        return false;
    }

    pub fn collision(&mut self, col_area: common::ScreenObjectArea) -> bool {
        for b in self.bomb.iter_mut()
            .filter(|b| b.in_flight && col_area.collides(b.area()))
            .take(1) {
            // once bomb has collided, it is no more, take care of it here
            b.in_flight = false;
            return true;
        }
        return false;
    }

    pub fn in_flight(&self) -> bool {
        self.bomb.iter().any(|&b| b.in_flight)
    }

    pub fn update(&mut self) {
        for b in &mut self.bomb {
            b.update();
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for b in self.bomb.iter().filter(|&b| b.in_flight) {
            image(&self.bomb_image, c.transform.trans(b.x, b.y), g);
        }
    }
}

