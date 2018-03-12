
use piston_window::*;

use common;
use common::win_image;

const FRAMES: usize = 6;
const SCROLL_SPEED: f64 = 1.0;
const CYCLE_PERIOD: u32 = 10;

pub struct Background {
    background_image: [G2dTexture; FRAMES],
    frame: u32,
    y: f64,
}

impl Background {
    pub fn new(window: &mut PistonWindow) -> Background {
        Background {
            background_image: [
                win_image(window, "stars-0.png"),
                win_image(window, "stars-1.png"),
                win_image(window, "stars-2.png"),
                win_image(window, "stars-3.png"),
                win_image(window, "stars-4.png"),
                win_image(window, "stars-5.png")],
            frame: 0,
            y: 0.0,
        }
    }
        
                     
    pub fn update(&mut self) {
        self.frame += 1;
        self.y += SCROLL_SPEED;
        if self.y >= common::SCREEN_HEIGHT {
            self.y -= common::SCREEN_HEIGHT;
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        let anim_frame = (self.frame / CYCLE_PERIOD) as usize % FRAMES;
        image(&self.background_image[anim_frame],
              c.transform.trans(0.0, self.y - common::SCREEN_HEIGHT), g);
        image(&self.background_image[anim_frame],
              c.transform.trans(0.0, self.y), g);
    }
}
