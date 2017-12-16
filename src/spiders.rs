
use piston_window::*;
use music;
use common;
use common::win_image;
use mother::Mother;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use bombs::Bombs;
use rand;
use rand::Rng;
use std::f64::consts::PI;

const NUMBER_OF_SPIDERS: usize = 45;
const MAX_SPIDERS_IN_FLIGHT: u32 = 8;
const SPIDER_WIDTH: f64 = 30.0;
const SPIDER_HEIGHT: f64 = 40.0;
const SPIDER_PERIOD: i32 = 20;
const SPIDER_ROTATE_SPEED: f64 = 0.05;
const SWOOP_SPEED: f64 = 5.0;
const FRAMES_BETWEEN_LAUNCHES: i32 = 30;
const FLIGHT_SPIDER_Y_MAX: f64 = 610.0;
const FLIGHT_SPIDER_Y_MIN: f64 = 200.0;
const SPEED_SLOW: f64 = 2.0;
const SPEED_MEDIUM: f64 = 3.0;
const SPEED_FAST: f64 = 4.0;
const SPEED: [[(f64, f64); 3]; 3] =
    [[(SPEED_SLOW, SPEED_SLOW), (SPEED_SLOW / 2.0, SPEED_SLOW), (0.0, SPEED_SLOW)],
     [(SPEED_MEDIUM * 2.0, SPEED_MEDIUM), (SPEED_MEDIUM, SPEED_MEDIUM), (SPEED_MEDIUM / 2.0, SPEED_MEDIUM)],
     [(SPEED_FAST, 0.0), (SPEED_FAST * 2.0, SPEED_FAST), (SPEED_FAST, SPEED_FAST)]];

#[derive(Copy, Clone)]
enum State {
    Nestle,
    Swoop(f64, f64),
    Seek(f64, f64, Option<common::TargetBrick>),
    Descend(common::TargetBrick),
    Grab(f64, f64),
    Carry(f64, f64, Option<common::TargetBrick>),
    Release(f64, f64),
    Dead(usize),
}

#[derive(Copy, Clone)]
enum Type {
    Slow,
    Medium,
    Fast,
}

enum DirRequired {
    Up,
    Down,
    Any,
}

#[derive(Copy, Clone)]
struct Spider {
    spider_type: Type,
    state: State,
    x: f64,
    y: f64,
    next_dir_change: u32,
    next_bomb_release: u32,
}

impl Spider {
    fn new() -> Spider {
        Spider {spider_type: Type::Medium, state: State::Nestle, x: 0.0, y: 0.0,
            next_dir_change: 0, next_bomb_release: 0}
    }

    fn launch(&mut self, mother: &Mother) -> bool {
        if let Some(d) = mother.launch_dir() {
            // transform coords from relative to mother to centre of spider
            // ready for swoop
            let (mother_x, mother_y) = mother.location();
            self.x += mother_x + 0.5 * SPIDER_WIDTH * 0.2;
            self.y += mother_y - 0.5 * SPIDER_WIDTH * 0.2;
            self.state = State::Swoop(0.0, d);
            true
        }
        else {
            false
        }
    }

    fn trajectory_reaches_target(&self, x_target: f64, y_target: f64, x_vel: f64, y_vel: f64) -> bool {
        (y_target - self.y).signum() == y_vel.signum() &&
        (((x_target - self.x) / x_vel) * y_vel + self.y - y_target).abs() < y_vel.abs()
    }

    fn new_trajectory_for_target(&self, x_target: f64, y_target: f64) -> Option<(f64, f64)> {
        let s = self.spider_type as usize;
        for i in 0..3 {
            let (x_vel, y_vel) = SPEED[s][i];
            if self.trajectory_reaches_target(x_target, y_target, x_vel, y_vel) {
                return Some((x_vel, y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, -x_vel, y_vel) {
                return Some((-x_vel, y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, -x_vel, -y_vel) {
                return Some((-x_vel, -y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, x_vel, -y_vel) {
                return Some((x_vel, -y_vel));
            }
        }
        return None;
    }

    fn random_sign(n: f64) -> f64 {
        if rand::thread_rng().gen() {-n} else {n}
    }

    fn random_vel(&self, dr: DirRequired) -> (f64, f64) {
        let (x_vel, y_vel) = SPEED[self.spider_type as usize][rand::thread_rng().gen_range(0, 3)];
        match dr {
            DirRequired::Down => {
                let y_vel = if y_vel == 0.0 {SPEED_FAST} else {y_vel};
                (Spider::random_sign(x_vel), y_vel)
            },
            DirRequired::Up => {
                let y_vel = if y_vel == 0.0 {SPEED_FAST} else {y_vel};
                (Spider::random_sign(x_vel), -y_vel)
            },
            DirRequired::Any => {
                (Spider::random_sign(x_vel), Spider::random_sign(y_vel))
            },
        }
    }

    fn aimless_wandering(&mut self, x_vel: f64, y_vel: f64) -> (f64, f64) {
        if self.next_dir_change == 0 {
            self.next_dir_change = rand::thread_rng().gen_range(100, 200);
        }

        self.next_dir_change -= 1;

        let (x_vel, y_vel) = if self.next_dir_change == 0
            {self.random_vel(DirRequired::Any)} else {(x_vel, y_vel)};

        let x_vel = if self.x + x_vel > common::SCREEN_WIDTH - SPIDER_WIDTH ||
            self.x + x_vel < 0.0 {- x_vel} else {x_vel};

        let y_vel = if (self.y + y_vel > FLIGHT_SPIDER_Y_MAX - SPIDER_HEIGHT && y_vel > 0.0) ||
            (self.y + y_vel < FLIGHT_SPIDER_Y_MIN && y_vel < 0.0)
            {- y_vel} else {y_vel};
        (x_vel, y_vel)
    }

    fn drop_bomb(&mut self, bombs: &mut Bombs) {
        if self.next_bomb_release == 0 {
            self.next_bomb_release = rand::thread_rng().gen_range(50, 200);
        }
        self.next_bomb_release -= 1;
        if self.next_bomb_release == 0 {
            bombs.release(self.x + SPIDER_WIDTH / 2.0, self.y + SPIDER_HEIGHT);
        }
    }

    fn update(&mut self, base_bricks: &mut BaseBricks, letter_bricks: &mut LetterBricks,
        bombs: &mut Bombs) {
        match self.state {
            State::Swoop(n, r) => {
                if n < 1.0 {
                    self.state = State::Swoop(n + SPIDER_ROTATE_SPEED, r);
                    self.x -= (PI + PI * n * r).sin() * SWOOP_SPEED;
                    self.y += (PI + PI * n * r).cos() * SWOOP_SPEED;
                }
                else {
                    // transform from centre coords to top left coords
                    self.x = (self.x - SPIDER_WIDTH * 0.5).floor();
                    self.y = (self.y - SPIDER_HEIGHT * 0.5).floor();
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Down);
                    self.state = State::Seek(x_vel, y_vel, None);
                }
            },
            State::Seek(x_vel, y_vel, target) => {
                match target {
                    Some(target_brick) => {
                        // caculate adjusted target x,y for top left of spider + a little bit of
                        // descend space
                        let adj_x = target_brick.x - 9.0;
                        let adj_y = target_brick.y - 40.0;

                        // have we reached the target (or close enough)?
                        if (self.x - adj_x).abs() < x_vel.abs() &&
                           (self.y - adj_y).abs() < y_vel.abs() {
                            self.x = adj_x;
                            self.y = adj_y;
                            self.state = State::Descend(target_brick);
                        }
                        // Are we already on a trajectory to reach the target?
                        else if self.trajectory_reaches_target(adj_x, adj_y, x_vel, y_vel) {
                            // stay as we are!
                            self.x += x_vel;
                            self.y += y_vel;
                        }
                        // can we take a trajectory now that will reach the target?
                        else if let Some((new_x_vel, new_y_vel)) = self.new_trajectory_for_target(adj_x, adj_y) {
                            // change direction
                            self.state = State::Seek(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        else {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Seek(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                    },
                    None => {
                        let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                        let new_target = base_bricks.request_target();
                        self.state = State::Seek(new_x_vel, new_y_vel, new_target);
                        self.x += new_x_vel;
                        self.y += new_y_vel;
                    },
                }
                self.drop_bomb(bombs);
            },
            State::Descend(target) => {
                self.y += 1.0;
                if self.y >= target.y - SPIDER_HEIGHT + 8.0 {
                    base_bricks.take_target(target.brick_id);
                    self.state = State::Grab(0.0, if rand::thread_rng().gen() {-1.0} else {1.0});
                    // transform coords to centre of spider
                    self.x += SPIDER_WIDTH * 0.5;
                    self.y += SPIDER_HEIGHT * 0.5;
                    common::play_sound(&common::Sound::TakeBrick);
                }
            },
            State::Grab(n, r) => {
                if n < 1.0 {
                    self.state = State::Grab(n + SPIDER_ROTATE_SPEED, r);
                }
                else {
                    // transform from centre coords to top left coords
                    self.x -= SPIDER_WIDTH * 0.5;
                    self.y -= SPIDER_HEIGHT * 0.5;
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Up);
                    self.state = State::Carry(x_vel, y_vel, None);
                }
            },
            State::Carry(x_vel, y_vel, target) => {
                match target {
                    Some(target_brick) => {
                        // caculate adjusted target x,y for top left of spider
                        let adj_x = target_brick.x - 9.0;
                        let adj_y = target_brick.y;

                        // have we reached the target (or close enough)?
                        if (self.x - adj_x).abs() < x_vel.abs() &&
                           (self.y - adj_y).abs() < y_vel.abs() {
                            letter_bricks.fill_target(target_brick.brick_id);
                            self.state = State::Release(0.0, if rand::thread_rng().gen() {-1.0} else {1.0});
                            self.x = adj_x + SPIDER_WIDTH * 0.5;
                            self.y = adj_y + SPIDER_HEIGHT * 0.5;
                            common::play_sound(&common::Sound::DepositBrick);
                        }
                        // Are we already on a trajectory to reach the target?
                        else if self.trajectory_reaches_target(adj_x, adj_y, x_vel, y_vel) {
                            // stay as we are!
                            self.x += x_vel;
                            self.y += y_vel;
                        }
                        // can we take a trajectory now that will reach the target?
                        else if let Some((new_x_vel, new_y_vel)) = self.new_trajectory_for_target(adj_x, adj_y) {
                            // change direction
                            self.state = State::Carry(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        else {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Carry(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                    },
                    None => {
                        let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                        let new_target = letter_bricks.request_target();
                        self.state = State::Carry(new_x_vel, new_y_vel, new_target);
                        self.x += new_x_vel;
                        self.y += new_y_vel;
                    },
                }
                self.drop_bomb(bombs);
            },
            State::Release(n, r) => {
                if n < 1.0 {
                    self.state = State::Release(n + SPIDER_ROTATE_SPEED, r);
                }
                else {
                    // transform from centre coords to top left coords
                    self.x -= SPIDER_WIDTH * 0.5;
                    self.y -= SPIDER_HEIGHT * 0.5;
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Down);
                    self.state = State::Seek(x_vel, y_vel, None);
                }
            },
            State::Dead(i) => {
                // care much that this carries on counting for the rest of the screen?
                self.state = State::Dead(i + 1);
            }
            _ => {}
        }
    }

    fn collision(&self, col_area: &common::ScreenObjectArea) -> bool {
        let collides =  col_area.collides(
            common::ScreenObjectArea::new(self.x, self.y, SPIDER_WIDTH, SPIDER_HEIGHT));

        match self.state {
            State::Seek(_, _, _) => {collides},
            State::Carry(_, _, _) => {collides},
            State::Release(_, _) => {collides},
            _ => {false}
        }
    }
}

pub struct Spiders {
    spider_image_empty: [[G2dTexture; 4]; 3],
    spider_image_laden: [[G2dTexture; 4]; 3],
    spider_image_explosion: [G2dTexture; 4],
    spiders_left: u32,
    spiders_in_flight: u32,
    next_spider_launch: usize,
    last_launch_frame: i32,
    spider: [Spider; NUMBER_OF_SPIDERS],
}

impl Spiders {
    pub fn new(window: &mut PistonWindow) -> Spiders {
        let mut new_spiders = Spiders {
            spider_image_empty:
                [[win_image(window, "spider_empty1_1.png"),
                  win_image(window, "spider_empty1_2.png"),
                  win_image(window, "spider_empty1_3.png"),
                  win_image(window, "spider_empty1_4.png")],
                 [win_image(window, "spider_empty2_1.png"),
                  win_image(window, "spider_empty2_2.png"),
                  win_image(window, "spider_empty2_3.png"),
                  win_image(window, "spider_empty2_4.png")],
                 [win_image(window, "spider_empty3_1.png"),
                  win_image(window, "spider_empty3_2.png"),
                  win_image(window, "spider_empty3_3.png"),
                  win_image(window, "spider_empty3_4.png")]],
            spider_image_laden:
                [[win_image(window, "spider_laden1_1.png"),
                  win_image(window, "spider_laden1_2.png"),
                  win_image(window, "spider_laden1_3.png"),
                  win_image(window, "spider_laden1_4.png")],
                 [win_image(window, "spider_laden2_1.png"),
                  win_image(window, "spider_laden2_2.png"),
                  win_image(window, "spider_laden2_3.png"),
                  win_image(window, "spider_laden2_4.png")],
                 [win_image(window, "spider_laden3_1.png"),
                  win_image(window, "spider_laden3_2.png"),
                  win_image(window, "spider_laden3_3.png"),
                  win_image(window, "spider_laden3_4.png")]],
            spider_image_explosion:
                [win_image(window, "spider_explosion1.png"),
                 win_image(window, "spider_explosion2.png"),
                 win_image(window, "spider_explosion3.png"),
                 win_image(window, "spider_explosion4.png")],
            spiders_left: NUMBER_OF_SPIDERS as u32,
            spiders_in_flight: 0,
            next_spider_launch: 0,
            last_launch_frame: 0,
            spider: [Spider::new(); NUMBER_OF_SPIDERS]};
        for i in 0..NUMBER_OF_SPIDERS {
            if i < 11 {
                new_spiders.spider[i].spider_type = Type::Slow;
            }
            else if i > 35 {
                new_spiders.spider[i].spider_type = Type::Fast;
            }
            // nestle spiders are 6 by 8
            // for nestle spiders, x and y are relative to mother
            new_spiders.spider[i].y = ((i / 15) * 8) as f64 - 16.0;
            new_spiders.spider[i].x = ((i % 15) * 6 + 5) as f64;
        }
        new_spiders
    }

    pub fn reset(&mut self) {
        for i in 0..NUMBER_OF_SPIDERS {
            self.spider[i].y = ((i / 15) * 8) as f64 - 16.0;
            self.spider[i].x = ((i % 15) * 6 + 5) as f64;
            self.spider[i].state = State::Nestle;
            self.spider[i].next_dir_change = 0;
            self.spider[i].next_bomb_release = 0;
            self.spiders_left = NUMBER_OF_SPIDERS as u32;
            self.spiders_in_flight = 0;
            self.next_spider_launch = 0;
            self.last_launch_frame = 0;
        }
    }

    pub fn update(&mut self, mother: &Mother, base_bricks: &mut BaseBricks,
                  letter_bricks: &mut LetterBricks, bombs: &mut Bombs, frame_count: i32) {
        if self.spiders_in_flight < MAX_SPIDERS_IN_FLIGHT &&
           self.next_spider_launch < NUMBER_OF_SPIDERS &&
           frame_count - self.last_launch_frame >= FRAMES_BETWEEN_LAUNCHES &&
           self.spider[self.next_spider_launch].launch(mother) {
            self.next_spider_launch += 1;
            self.spiders_in_flight += 1;
            self.last_launch_frame = frame_count;
        }
        for i in 0..NUMBER_OF_SPIDERS {
            self.spider[i].update(base_bricks, letter_bricks, bombs)
        }
    }

    pub fn collision(&mut self, col_area: common::ScreenObjectArea) -> Option<usize> {
        for i in 0..NUMBER_OF_SPIDERS {
            if self.spider[i].collision(&col_area) {
                return Some(i);
            }
        }
        return None;
    }

    pub fn target_brick_id(&self, spider_id: usize) -> Option<usize> {
        match self.spider[spider_id].state {
            State::Seek(_, _, target) => {
                if let Some(target_brick) = target {
                    Some(target_brick.brick_id)
                }
                else {
                    None
                }
            },
            State::Carry(_, _, target) => {
                if let Some(target_brick) = target {
                    Some(target_brick.brick_id)
                }
                else {
                    None
                }
            },
            _ => {None},
        }
    }

    pub fn carrying(&self, spider_id: usize) -> bool {
        match self.spider[spider_id].state {
            State::Carry(_, _, _) => {true},
            _ => {false},
        }
    }

    pub fn kill(&mut self, spider_id: usize) {
        self.spider[spider_id].state = State::Dead(0);
        self.spiders_in_flight -= 1;
        self.spiders_left -= 1;
        common::play_sound(&common::Sound::SpiderExplode);
    }

    pub fn spiders_remain(&self) -> bool {
        self.spiders_left > 0
    }

    pub fn render(&self, mother: &Mother, c: Context, g: &mut G2d, frame_count: i32) {
        let anim_frame = ((frame_count % SPIDER_PERIOD) / (SPIDER_PERIOD / 4)) as usize;
        for i in 0..NUMBER_OF_SPIDERS {
            let spider = self.spider[i];
            let type_i = match spider.spider_type {
                Type::Slow => 0,
                Type::Medium => 1,
                Type::Fast => 2
            };
            match spider.state {
                State::Nestle => {
                    let (mother_x, mother_y) = mother.location();
                    let x = spider.x + mother_x;
                    let y = spider.y + mother_y;
                    image(&self.spider_image_empty[type_i][anim_frame],
                          c.transform.trans(x, y).flip_v().zoom(0.2), g);
                },
                State::Swoop(n, r) => {
                    image(&self.spider_image_empty[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y)
                          .rot_rad(PI + PI * n * r)
                          .trans(- SPIDER_WIDTH * 0.5 * (0.2 + 0.8 * n), - SPIDER_HEIGHT * 0.5 * (0.2 + 0.8 * n))
                          .zoom(0.2 + 0.8 * n),
                          g);
                },
                State::Seek(_, _, _) => {
                    image(&self.spider_image_empty[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y), g);
                },
                State::Descend(_) => {
                    image(&self.spider_image_empty[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y), g);
                },
                State::Grab(n, r) => {
                    image(&self.spider_image_laden[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y)
                          .rot_rad(PI + PI * n * r)
                          .trans(- SPIDER_WIDTH * 0.5, - SPIDER_HEIGHT * 0.5),
                          g);
                },
                State::Carry(_, _, _) => {
                    image(&self.spider_image_laden[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y), g);
                },
                State::Release(n, r) => {
                    image(&self.spider_image_empty[type_i][anim_frame],
                          c.transform.trans(spider.x, spider.y)
                          .rot_rad(PI + PI * n * r)
                          .trans(- SPIDER_WIDTH * 0.5, - SPIDER_HEIGHT * 0.5),
                          g);
                },
                State::Dead(i) => {
                    if i < 20 {
                        image(&self.spider_image_explosion[i / 5],
                              c.transform.trans(spider.x, spider.y), g);
                    }
                },
                _ => {}
            };
        }
    }
}