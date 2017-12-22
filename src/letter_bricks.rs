use piston_window::*;
use common;
use rand;
use rand::Rng;

const LETTER_BRICKS_Y: i32 = 140;
const LETTER_BRICKS_X: i32 = 97;
const BRICK_HEIGHT: i32 = 10;
const BRICK_WIDTH: i32 = 15;
const BRICKS_QTY: usize = 86;
const MAX_COMBINED_ROW_BRICKS: usize = 23;

pub struct LetterBrick {
    row: i32,
    col: i32,
    filled: bool,
    targetted: bool,
}

impl LetterBrick {
    fn new(row: i32, col: i32) -> LetterBrick {
        LetterBrick {row, col, filled: false, targetted: false}
    }
}

pub struct LetterBricks {
    letter_brick: [LetterBrick; BRICKS_QTY],
    letter_range: [[[usize; 2]; 6]; 3],
    qty_filled: usize,
    brick_image: G2dTexture,
}

impl LetterBricks {
    pub fn new(window: &mut PistonWindow) -> LetterBricks {
        LetterBricks {
        letter_brick:
            [
            LetterBrick::new(0, 0), // 0
            LetterBrick::new(0, 1),
            LetterBrick::new(0, 2),
            LetterBrick::new(0, 3),
            LetterBrick::new(0, 4),
            LetterBrick::new(0, 5),
            LetterBrick::new(0, 6),
            LetterBrick::new(0, 7),
            LetterBrick::new(0, 8),
            LetterBrick::new(0, 9),
            LetterBrick::new(0, 11), // 10
            LetterBrick::new(0, 12),
            LetterBrick::new(0, 13),
            LetterBrick::new(0, 14),
            LetterBrick::new(0, 15),
            LetterBrick::new(0, 16),
            LetterBrick::new(0, 18), // 16
            LetterBrick::new(0, 19),
            LetterBrick::new(0, 25),
            LetterBrick::new(0, 26),
            LetterBrick::new(1, 0), // 20
            LetterBrick::new(1, 1),
            LetterBrick::new(1, 13), // 22
            LetterBrick::new(1, 14),
            LetterBrick::new(1, 18), // 24
            LetterBrick::new(1, 19),
            LetterBrick::new(1, 20),
            LetterBrick::new(1, 21),
            LetterBrick::new(1, 25),
            LetterBrick::new(1, 26),
            LetterBrick::new(2, 0), // 30 
            LetterBrick::new(2, 1),
            LetterBrick::new(2, 2),
            LetterBrick::new(2, 3),
            LetterBrick::new(2, 4),
            LetterBrick::new(2, 5),
            LetterBrick::new(2, 6),
            LetterBrick::new(2, 7),
            LetterBrick::new(2, 13), // 38
            LetterBrick::new(2, 14),
            LetterBrick::new(2, 18), // 40
            LetterBrick::new(2, 19),
            LetterBrick::new(2, 20),
            LetterBrick::new(2, 21),
            LetterBrick::new(2, 22),
            LetterBrick::new(2, 25),
            LetterBrick::new(2, 26),
            LetterBrick::new(3, 0), // 47
            LetterBrick::new(3, 1),
            LetterBrick::new(3, 2),
            LetterBrick::new(3, 3),
            LetterBrick::new(3, 4),
            LetterBrick::new(3, 5),
            LetterBrick::new(3, 6),
            LetterBrick::new(3, 7),
            LetterBrick::new(3, 13), // 55
            LetterBrick::new(3, 14),
            LetterBrick::new(3, 18), // 57
            LetterBrick::new(3, 19),
            LetterBrick::new(3, 22),
            LetterBrick::new(3, 23),
            LetterBrick::new(3, 24),
            LetterBrick::new(3, 25),
            LetterBrick::new(3, 26),
            LetterBrick::new(4, 0), // 64
            LetterBrick::new(4, 1),
            LetterBrick::new(4, 13), // 66
            LetterBrick::new(4, 14),
            LetterBrick::new(4, 18), // 68
            LetterBrick::new(4, 19),
            LetterBrick::new(4, 23),
            LetterBrick::new(4, 24),
            LetterBrick::new(4, 25),
            LetterBrick::new(4, 26),
            LetterBrick::new(5, 0), // 74
            LetterBrick::new(5, 1),
            LetterBrick::new(5, 11), // 76
            LetterBrick::new(5, 12),
            LetterBrick::new(5, 13),
            LetterBrick::new(5, 14),
            LetterBrick::new(5, 15),
            LetterBrick::new(5, 16),
            LetterBrick::new(5, 18), // 82
            LetterBrick::new(5, 19),
            LetterBrick::new(5, 25),
            LetterBrick::new(5, 26),
        ],
        letter_range: [[[0, 9], [20, 21], [30, 37], [47, 54], [64, 65], [74, 75]],
                       [[10, 15], [22, 23], [38, 39], [55, 56], [66, 67], [76, 81]],
                       [[16, 19],  [24, 29], [40, 46], [57, 63], [68, 73], [82, 85]]],
        qty_filled: 0,
        brick_image: common::win_image(window, "letterbrick.png")}
    }

    pub fn reset(&mut self) {
        self.qty_filled = 0;
        for b in &mut self.letter_brick.iter_mut() {
            b.targetted = false;
            b.filled = false;
        }
    }

    fn row_has_gaps(&self, letter: usize, row: usize) -> bool {
        for i in self.letter_range[letter][row][0]..self.letter_range[letter][row][1] + 1 {
            if ! self.letter_brick[i].filled {
                return true;
            }
        }
        return false;
    }

    pub fn fill_target(&mut self, brick_id: usize) {
        self.letter_brick[brick_id].filled = true;
        self.letter_brick[brick_id].targetted = false;
        self.qty_filled += 1;
    }

    pub fn untarget(&mut self, brick_id: usize) {
        self.letter_brick[brick_id].targetted = false;
    }

    pub fn request_target(&mut self) -> Option<common::TargetBrick> {
        // -> brick x,y brick id
        let mut target_list = [0; MAX_COMBINED_ROW_BRICKS];  
        let mut list_len: usize = 0;

        if self.qty_filled < BRICKS_QTY {
            for i in 0..3 {
                for j in 0..6 {
                    if self.row_has_gaps(i, j) {
                        let from = self.letter_range[i][j][0];
                        let to = self.letter_range[i][j][1];
                        for k in from..to + 1 {
                            if ! (self.letter_brick[k].filled || self.letter_brick[k].targetted) {
                                // fixme: this is too terrible for words
                                if (k == from || ! self.letter_brick[k - 1].targetted || self.letter_brick[k].col - self.letter_brick[k - 1].col != 1) &&
                                   (k == to || ! self.letter_brick[k + 1].targetted || self.letter_brick[k + 1].col - self.letter_brick[k].col != 1) {
                                    target_list[list_len] = k;
                                    list_len += 1;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        if list_len > 0 {
            let id = target_list[rand::thread_rng().gen_range(0, list_len)];
            let x = (LETTER_BRICKS_X + self.letter_brick[id].col * BRICK_WIDTH) as f64;
            let y = (LETTER_BRICKS_Y + self.letter_brick[id].row * BRICK_HEIGHT) as f64;
            self.letter_brick[id].targetted = true;
            Some(common::TargetBrick {x, y, brick_id: id})
        }
        else {
            None
        }
    }

    pub fn complete(&self) -> bool {
        self.qty_filled == BRICKS_QTY
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for b in self.letter_brick.iter().filter(|&b| b.filled) {
            let x = LETTER_BRICKS_X + b.col * BRICK_WIDTH;
            let y = LETTER_BRICKS_Y + b.row * BRICK_HEIGHT;
            image(&self.brick_image, c.transform.trans(x as f64, y as f64), g);
        }
    }
}
