use piston_window::*;
use common;
use rand;
use rand::Rng;

const BASE_BRICKS_Y: i32 = 630;
const BRICKS_HOME_X: i32 = 60;
const BRICKS_SPEED: i32 = 2;
const BRICK_HEIGHT: i32 = 10;
const BRICK_WIDTH: i32 = 15;

pub struct BaseBricks {
    x: i32,
    filled: [bool; 4 * 4 * 3],
    targetted: [bool; 4 * 4 * 3],
    qty_filled: u32,
    brick_image: G2dTexture,
}

impl BaseBricks {
    pub fn new(window: &mut PistonWindow) -> BaseBricks {
        BaseBricks {x: 60, filled: [false; 4 * 4 * 3], targetted: [false; 4 * 4 * 3],
            qty_filled: 0, brick_image: common::win_image(window, "brick.png")}
    }

    pub fn reset(&mut self) {
        self.qty_filled = 0;
    }

    fn brick_id(pile: usize, col: usize, row: usize) -> usize {
        pile * 16 + row * 4 + col
    }

    fn row_has_bricks(&self, pile: usize, row: usize) -> bool {
        let i = pile * 16 + row * 4;
        self.filled[i] || self.filled[i + 1] || self.filled[i + 2] || self.filled[i + 3]
    }

    pub fn update(&mut self) {
        if self.qty_filled == 0 {
            self.x = -600;
            self.filled = [true; 4 * 4 * 3];
            self.targetted = [false; 4 * 4 * 3];
            self.qty_filled = 4 * 4 * 3;
        }
        else if self.x < BRICKS_HOME_X {
            self.x += BRICKS_SPEED;
        }
    }

    pub fn request_target(&mut self) -> Option<common::TargetBrick> {
        // build up a list of all bricks that are in the top row of their pile that
        // are not already targetted and are not next to another that is targetted
        let mut target_list = [0; 12];
        let mut list_len: usize = 0;
        if self.qty_filled > 0 && self.x >= BRICKS_HOME_X {
            for i in 0..3 {
                for j in 0..4 {
                    if self.row_has_bricks(i, j) {
                        let id0 = BaseBricks::brick_id(i, 0, j);
                        let t0 = self.targetted[id0];
                        let t1 = self.targetted[id0 + 1];
                        let t2 = self.targetted[id0 + 2];
                        let t3 = self.targetted[id0 + 3];
                        if (! t0) && (! t1) && self.filled[id0] {
                            target_list[list_len] = id0;
                            list_len += 1;
                        }
                        if (! t0) && (! t1) && (! t2) && self.filled[id0 + 1] {
                            target_list[list_len] = id0 + 1;
                            list_len += 1;
                        }
                        if (! t1) && (! t2) && (! t3) && self.filled[id0 + 2] {
                            target_list[list_len] = id0 + 2;
                            list_len += 1;
                        }
                        if (! t2) && (! t3) && self.filled[id0 + 3] {
                            target_list[list_len] = id0 + 3;
                            list_len += 1;
                        }
                        break;
                    }
                }
            }
        }
        // choose a random target from the list
        if list_len > 0 {
            let id = target_list[rand::thread_rng().gen_range(0, list_len)];
            let x = (self.x + (id as i32 / 16) * 210 + (id as i32 % 4) * BRICK_WIDTH) as f64;
            let y = (BASE_BRICKS_Y + ((id as i32 % 16) / 4) * BRICK_HEIGHT) as f64;
            self.targetted[id] = true;
            Some(common::TargetBrick {x, y, brick_id: id})
        }
        else {
            None
        }
    }

    pub fn take_target(&mut self, brick_id: usize) {
        self.filled[brick_id] = false;
        self.targetted[brick_id] = false;
        self.qty_filled -= 1;
    }

    pub fn untarget(&mut self, brick_id: usize) {
        self.targetted[brick_id] = false;
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for i in 0..3 {
            for j in 0..4 {
                for k in 0..4 {
                    if self.filled[BaseBricks::brick_id(i, j, k) as usize] {
                        let y = BASE_BRICKS_Y + (k as i32) * BRICK_HEIGHT;
                        let x = self.x + i as i32 * 210 + j as i32 * BRICK_WIDTH;
                        image(&self.brick_image, c.transform.trans(x as f64, y as f64), g);
                    }
                }
            }
        }
    }
}
