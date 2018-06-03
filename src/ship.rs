
use piston_window::*;

use missile;
use common;
use soundfx;
use animation::Animations;

const SHIP_WIDTH: f64 = 30.0;
const SHIP_HEIGHT: f64 = 40.0;
const SHIP_Y: f64 = 565.0;
const SHIP_SPEED: f64 = 5.0;
const LIVES: u32 = 4;
const LIVES_Y: f64 = common::SCREEN_HEIGHT - 2.0 - SHIP_HEIGHT / 2.0;
const LIVES_X: f64 = common::SCREEN_WIDTH - 2.0 - SHIP_WIDTH / 2.0;
const MIN_FRAMES_BEFORE_CHANGEOVER: u32 = common::UPDATE_FPS as u32 * 3 / 2;
const GRACE_PERIOD_FRAMES: u32 = common::UPDATE_FPS as u32;

enum ShipState {
    Alive(u32),
    WaitForChangeOver(u32),
    ChangeOver(f64),
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
        Ship{x: Ship::home_x(),
             state: ShipState::Alive(0),
             lives: LIVES,
             ship_image: [common::win_image(window, "ship1.png"),
                          common::win_image(window, "ship2.png"),
                          common::win_image(window, "ship3.png")],
             explosion_image: [common::win_image(window, "ship_explosion1.png"),
                               common::win_image(window, "ship_explosion2.png"),
                               common::win_image(window, "ship_explosion3.png"),
                               common::win_image(window, "ship_explosion4.png")]}
    }

    fn home_x() -> f64 {
        ((common::SCREEN_WIDTH - SHIP_WIDTH) / 2.0).floor()
    }

    pub fn reset(&mut self) {
        self.x = Ship::home_x();
        self.state = ShipState::Alive(0);
        self.lives = LIVES;
    }

    pub fn move_left(&mut self) {
        if let ShipState::Alive(_) = self.state {
            if self.x > 0.0 {
                self.x -= SHIP_SPEED;
            }
        }
    }

    pub fn move_right(&mut self) {
        if let ShipState::Alive(_) = self.state {
            if self.x < common::SCREEN_WIDTH - SHIP_WIDTH {
                self.x += SHIP_SPEED;
            }
        }
    }

    pub fn kill(&mut self, sound: &soundfx::SoundFx, animations: &mut Animations) {
        if let ShipState::Alive(_) = self.state {
            let x = self.x;
            // fixme:
            let explosion = [self.explosion_image[0].clone(),
                             self.explosion_image[1].clone(),
                             self.explosion_image[2].clone(),
                             self.explosion_image[3].clone()];
            animations.register(
                Box::new(move |frame, c, g| {
                    image(&explosion[(frame / 8) as usize], c.transform.trans(x, SHIP_Y), g);
                }),
                32);
            sound.ship_explode();
            self.state = ShipState::WaitForChangeOver(MIN_FRAMES_BEFORE_CHANGEOVER);
            self.x = Ship::home_x();
        }
    }

    pub fn award_extra_life(&mut self, sound: &soundfx::SoundFx, animations: &mut Animations) {
        self.lives += 1;
        animations.register_text(
            Box::new(move |frame, c, g, gl| {
                text::Text::new_color([1.0, 0.0, 0.0, 1.0 - (frame as f32 / 50.0)], 40).draw(
                    &format!("Extra Life!"),
                    gl,
                    &c.draw_state,
                    c.transform.trans(210.0, 450.0),
                    g
                    ).unwrap();
            }),
            50);
        sound.extra_life();
    }

    pub fn alive(&self) -> bool {
        if let ShipState::Alive(_) = self.state {true} else {false}
    }

    pub fn life_left(&self) -> bool {
        if self.lives == 0 {
            if let ShipState::WaitForChangeOver(_) = self.state {
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
            ShipState::WaitForChangeOver(n) => {
                if n > 0 {
                    self.state = ShipState::WaitForChangeOver(n - 1);
                }
            }
            ShipState::ChangeOver(n) => {
                if n < 1.0 {
                    self.state = ShipState::ChangeOver(n + 0.05);
                }
                else {
                    self.state = ShipState::Alive(GRACE_PERIOD_FRAMES);
                }
            },
            ShipState::Alive(n) => {
                if n > 0 {
                    self.state = ShipState::Alive(n - 1);
                }
            },
        }
    }

    pub fn launch_missile(&self, missile: &mut missile::Missile, sound: &soundfx::SoundFx) {
        if let ShipState::Alive(_) = self.state {
            missile.launch(self.x + (SHIP_WIDTH / 2.0).floor(), SHIP_Y, sound);
        }
    }

    pub fn waiting_for_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(_) => {true},
            _ => {false},
        }
    }

    pub fn enough_delay_for_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(n) => {n == 0},
            _ => {false}
        }
    }

    pub fn in_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(_) => {true},
            ShipState::ChangeOver(_) => {true},
            _ => {false},
        }
    }

    pub fn proceed_with_changeover(&mut self) {
        if self.lives > 0 {
            self.state = ShipState::ChangeOver(0.0);
            self.lives -= 1;
        }
    }

    pub fn protected(&self) -> bool {
        match self.state {
            ShipState::Alive(n) => {n > 0},
            _ => {false},
        }
    }

    fn life_x(life: u32) -> f64 {
        LIVES_X - life as f64 * (SHIP_WIDTH / 2.0 + 10.0)
    }

    pub fn render(&self, c: Context, g: &mut G2d, frame_count: u32) {
        match self.state {
            ShipState::Alive(_) => {
                let ship_pulse = frame_count % 30;
                image(&self.ship_image[(ship_pulse / 10) as usize],
                    c.transform.trans(self.x, SHIP_Y), g);
            },
            ShipState::ChangeOver(n) => {
                image(&self.ship_image[0],
                    c.transform.trans(self.x + (Ship::life_x(self.lives + 1) - self.x) * (1.0 - n),
                        SHIP_Y + (LIVES_Y - SHIP_Y) * (1.0 - n)).zoom(0.5 + 0.5 * n), g);
            },
            _ => {},
        }
        for i in 0..self.lives {
            image(&self.ship_image[0],
                c.transform.trans(Ship::life_x(i), LIVES_Y).zoom(0.5), g);
        }
    }
}

