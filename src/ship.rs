
use piston_window::*;
use common;

const SHIP_WIDTH: f64 = 30.0;
const SHIP_HEIGHT: f64 = 45.0;
const SHIP_Y: f64 = 560.0;
const SHIP_SPEED: f64 = 5.0;

pub struct Ship {
    x: f64,
    ship_image: [G2dTexture; 3],
}

impl Ship {
    pub fn new (window: &mut PistonWindow) -> Ship {
        Ship{x: ((common::SCREEN_WIDTH - SHIP_WIDTH) / 2.0).floor(),
             ship_image: [common::win_image(window, "ship1.png"),
                          common::win_image(window, "ship2.png"),
                          common::win_image(window, "ship3.png")]}
    }

    pub fn reset(&mut self) {
        self.x = ((common::SCREEN_WIDTH - SHIP_WIDTH) / 2.0).floor();
    }

    pub fn move_left(&mut self) {
        if self.x > 0.0 {
            self.x -= SHIP_SPEED;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < common::SCREEN_WIDTH - SHIP_WIDTH {
            self.x += SHIP_SPEED;
        }
    }

    pub fn launch_missile(&self, missile: &mut ::Missile) {
        missile.launch(self.x + (SHIP_WIDTH / 2.0).floor(), SHIP_Y);
    }

    pub fn render(&self, c: Context, g: &mut G2d, frame_count: i32) {
        let ship_pulse = frame_count % 30;
        image(&self.ship_image[(ship_pulse / 10) as usize], c.transform.trans(self.x, SHIP_Y), g);
    }
}

