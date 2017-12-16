use piston_window::*;
use common;

const MOTHER_Y: f64 = 100.0;
const MOTHER_WIDTH: f64 = 100.0;
const MOTHER_HEIGHT: f64 = 20.0;
const MOTHER_PERIOD: i32 = 32;
const MOTHER_SPEED: f64 = 2.0;

pub struct Mother {
    x: f64,
    vel: f64,
    mother_image1: G2dTexture,
    mother_image2: G2dTexture,
}

impl Mother {
    pub fn new(window: &mut PistonWindow) -> Mother {
        Mother {
            x: (common::SCREEN_WIDTH - MOTHER_WIDTH) * 0.5,
            vel: MOTHER_SPEED,
            mother_image1: common::win_image(window, "mother1.png"),
            mother_image2: common::win_image(window, "mother2.png")}
    }

    pub fn reset(&mut self) {
        self.x = (common::SCREEN_WIDTH - MOTHER_WIDTH) * 0.5;
    }

    pub fn update(&mut self) {
        self.x += self.vel;
        if self.x > common::SCREEN_WIDTH - MOTHER_WIDTH - MOTHER_SPEED || 
           self.x < MOTHER_SPEED {
            self.vel = - self.vel;
        }
    }

    pub fn location(&self) -> (f64, f64) {
        (self.x, MOTHER_Y)
    }

    pub fn launch_dir(&self) -> Option<f64> {
        // to launch a spider with a clean exit, we want to launch in the
        // opposite direction to the mother but only if there is enough space
        // between the mother and the edge of the screen to allow a swoop
        if (self.vel < 0.0 && self.x < common::SCREEN_WIDTH - 50.0 - MOTHER_WIDTH) ||
           (self.vel > 0.0 && self.x > 50.0) {
            Some(- self.vel.signum())
        }
        else {
            None
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, frame_count: i32) {
        let mother_image = if frame_count % MOTHER_PERIOD < MOTHER_PERIOD / 2
            {&self.mother_image1} else {&self.mother_image2};
        image(mother_image, c.transform.trans(self.x, MOTHER_Y), g);
    }
}
