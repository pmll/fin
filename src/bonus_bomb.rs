use rand;
use rand::Rng;
use std::f64::consts::PI;
use piston_window::*;

use letter_bricks::LetterBricks;
use common;
use soundfx::SoundFx;
use animation::Animations;

const START_Y: f64 = 120.0;
const BOMB_WIDTH: f64 = 50.0;
const BOMB_HEIGHT: f64 = 22.0;
const BOMB_SPEED: f64 = 2.0;
const BOMB_PERIOD: u32 = 10;
const BOMB_VALUE: [usize; 3] = [10, 30, 50];
const SCORE_MULTIPLIER: u32 = 20;

#[derive(Copy, Clone)]
enum BombType {
    Bonus10,
    Bonus30,
    Bonus50,
}

enum State {
    Dormant,
    InFlight,
}

pub struct BonusBomb {
    x: f64,
    y: f64,
    bomb_state: State,
    bomb_type: BombType,
    bomb_image: [[G2dTexture; 2]; 3],
}

impl BonusBomb {
    pub fn new(window: &mut PistonWindow) -> BonusBomb {
        BonusBomb {
            x: 0.0,
            y: 0.0,
            bomb_state: State::Dormant,
            bomb_type: BombType::Bonus10,
            bomb_image: [[common::win_image(window, "bonus10_1.png"),
                          common::win_image(window, "bonus10_2.png")],
                         [common::win_image(window, "bonus30_1.png"),
                          common::win_image(window, "bonus30_2.png")],
                         [common::win_image(window, "bonus50_1.png"),
                          common::win_image(window, "bonus50_2.png")]],
        } 
    }

    fn area(&self) -> common::ScreenObjectArea {
        // only the middle part of the bonus bomb is collidable
        common::ScreenObjectArea::new(self.x + 10.0, self.y, BOMB_WIDTH - 20.0, BOMB_HEIGHT)
    }

    fn in_flight(&self) -> bool {
        match self.bomb_state {
            State::InFlight => {true},
            _ => {false}
        }
    }

    pub fn reset(&mut self) {
        self.bomb_state = State::Dormant;
    }

    pub fn launch(&mut self, x: f64) {
        self.x = x - BOMB_WIDTH / 2.0;
        self.y = START_Y;
        self.bomb_state = State::InFlight;
        self.bomb_type =
            match rand::thread_rng().gen_range(0, 3) {
                0 => {BombType::Bonus10},
                1 => {BombType::Bonus30},
                _ => {BombType::Bonus50},
            };
    }

    pub fn collision(&mut self, col_area: common::ScreenObjectArea) -> bool {
        self.in_flight() && col_area.collides(self.area())
    }

    pub fn achieve_bonus(&mut self, letter_bricks: &mut LetterBricks, animations: &mut Animations) {
        self.bomb_state = State::Dormant;
        letter_bricks.initiate_removal(BOMB_VALUE[self.bomb_type as usize]);

        let x = self.x + BOMB_WIDTH / 2.0;
        let y = self.y + BOMB_HEIGHT / 2.0;
        let bomb_image = self.bomb_image[self.bomb_type as usize][0].clone();
        animations.register(
            Box::new(move |frame, c, g| {
                let n = (100 - frame) as f64 / 100.0;
                image(&bomb_image,
                      c.transform.trans(x, y)
                      .rot_rad(n * PI * 10.0)
                      .trans(- BOMB_WIDTH * 0.5 * n, - BOMB_HEIGHT * 0.5 * n)
                      .zoom(n),
                      g);
            }),
            100);
    }

    pub fn score(&self) -> u32 {
        BOMB_VALUE[self.bomb_type as usize] as u32 * SCORE_MULTIPLIER
    }

    pub fn update(&mut self, sound: &SoundFx) {
        match self.bomb_state {
            State::InFlight => {
                if (self.y - START_Y).floor() % 120.0 == 0.0 {
                    sound.bonus_bomb();
                }
                self.y += BOMB_SPEED;
                if self.y > common::SCREEN_HEIGHT {
                    self.bomb_state = State::Dormant;
                }
            },
            _ => {}
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, frame_count: u32) {
        let bt = self.bomb_type as usize;
        let an = ((frame_count / BOMB_PERIOD) % 2) as usize;

        match self.bomb_state {
            State::InFlight => {
                image(&self.bomb_image[bt][an], c.transform.trans(self.x, self.y), g);
            },
            _ => {},
        }
    }
}
