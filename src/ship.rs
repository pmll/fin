
use piston_window::*;
use common;

const SHIP_WIDTH: f64 = 30.0;
const SHIP_HEIGHT: f64 = 45.0;
const SHIP_Y: f64 = 560.0;
const SHIP_SPEED: f64 = 5.0;
const LIVES: u32 = 3;
const LIVES_Y: f64 = common::SCREEN_HEIGHT - 2.0 - SHIP_HEIGHT / 2.0;
const LIVES_X: f64 = common::SCREEN_WIDTH - 2.0 - SHIP_WIDTH / 2.0;

enum ShipState {
    Alive,
    Dead(usize),
    ChangeOver(u32),
}

pub struct Ship {
    x: f64,
    state: ShipState,
    lives: u32,
    ship_image: [G2dTexture; 3],
    explosion_image: [G2dTexture; 4],
}

impl Ship {
    pub fn new (window: &mut PistonWindow) -> Ship {
        Ship{x: ((common::SCREEN_WIDTH - SHIP_WIDTH) / 2.0).floor(),
             state: ShipState::Alive,
             lives: LIVES,
             ship_image: [common::win_image(window, "ship1.png"),
                          common::win_image(window, "ship2.png"),
                          common::win_image(window, "ship3.png")],
             explosion_image: [common::win_image(window, "spider_explosion1.png"),  // for now...
                               common::win_image(window, "spider_explosion2.png"),
                               common::win_image(window, "spider_explosion3.png"),
                               common::win_image(window, "spider_explosion4.png")]}
    }

    pub fn reset(&mut self) {
        self.x = ((common::SCREEN_WIDTH - SHIP_WIDTH) / 2.0).floor();
        self.state = ShipState::Alive;
        self.lives = LIVES;
    }

    pub fn move_left(&mut self) {
        if let ShipState::Alive = self.state {
            if self.x > 0.0 {
                self.x -= SHIP_SPEED;
            }
        }
    }

    pub fn move_right(&mut self) {
        if let ShipState::Alive = self.state {
            if self.x < common::SCREEN_WIDTH - SHIP_WIDTH {
                self.x += SHIP_SPEED;
            }
        }
    }

    pub fn kill(&mut self) {
        if let ShipState::Alive = self.state {
            self.state = ShipState::Dead(0);
            common::play_sound(&common::Sound::ShipExplode);
        }
    }

    pub fn alive(&self) -> bool {
        if let ShipState::Alive = self.state {true} else {false}
    }

    pub fn life_left(&self) -> bool {
        if self.lives == 0 {
            if let ShipState::ChangeOver(n) = self.state {
                return false;
            }
        }
        return true;
    }

    pub fn area(&self) -> common::ScreenObjectArea {
        common::ScreenObjectArea::new(self.x, SHIP_Y, SHIP_WIDTH, SHIP_HEIGHT)
    }

    pub fn update(&mut self) {
        match self.state {
            ShipState::Dead(n) => {
                if n < 31 {
                    self.state = ShipState::Dead(n + 1);
                }
                else {
                    self.state = ShipState::ChangeOver(0);
                }
            },
            ShipState::ChangeOver(n) => {
                // will do something more elaborate with the changeover, eventually...
                if self.lives > 0 {
                    self.lives -= 1;
                    self.state = ShipState::Alive;
                }
            },
            _ => {},
        }
    }

    pub fn launch_missile(&self, missile: &mut ::Missile) {
        if let ShipState::Alive = self.state {
            missile.launch(self.x + (SHIP_WIDTH / 2.0).floor(), SHIP_Y);
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, frame_count: i32) {
        match self.state {
            ShipState::Alive => {
                let ship_pulse = frame_count % 30;
                image(&self.ship_image[(ship_pulse / 10) as usize],
                    c.transform.trans(self.x, SHIP_Y), g);
            },
            ShipState::Dead(n) => {
                if n < 32 {
                    image(&self.explosion_image[n / 8],
                        c.transform.trans(self.x, SHIP_Y), g);
                }
            },
            _ => {},        // possibly will have an animation during changeover
        }
        for i in 0..self.lives {
            image(&self.ship_image[0],
                c.transform.trans(LIVES_X - i as f64 * (SHIP_WIDTH / 2.0 + 10.0), LIVES_Y).zoom(0.5), g);
        }
    }
}

