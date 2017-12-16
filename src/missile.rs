use piston_window::*;
use common;
use music;

const MISSILE_WIDTH: f64 = 3.0;
const MISSILE_HEIGHT: f64 = 15.0;
const MISSILE_SPEED: f64 = 12.0;

pub struct Missile {
    x: f64,
    y: f64,
    in_flight: bool,
    missile_image: G2dTexture,
}

impl Missile {
    pub fn new(window: &mut PistonWindow) -> Missile {
        Missile {x: 0.0, y: 0.0, in_flight: false,
            missile_image: common::win_image(window, "missile.png")}
    }

    pub fn reset(&mut self) {
        self.in_flight = false;
    }

    pub fn launch(&mut self, from_x: f64, from_y: f64) {
        if ! self.in_flight {
            self.x = from_x - (MISSILE_WIDTH / 2.0).floor();
            self.y = from_y - MISSILE_HEIGHT;
            self.in_flight = true;
            common::play_sound(&common::Sound::Fire);
        }
    }

    pub fn update(&mut self) {
        if self.in_flight {
            self.y -= MISSILE_SPEED;
        }
        if self.y < 0.0 {
            self.in_flight = false;
        }
    }

    pub fn area(&self) -> common::ScreenObjectArea {
        common::ScreenObjectArea::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT)
    }

    pub fn flying(&self) -> bool {
        self.in_flight
    }

    pub fn terminate_flight(&mut self) {
        self.in_flight = false;
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        if self.in_flight {
            image(&self.missile_image, c.transform.trans(self.x, self.y), g);
        }
    }
}

