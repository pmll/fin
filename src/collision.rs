use spiders::Spiders;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use missile::Missile;
use bombs::Bombs;
use ship::Ship;

const SPIDER_SCORE: [u32; 3] = [40, 80, 200];

// a collision means occupying the same space in the same frame
// that may turn out to be too naive but it will do for now
pub fn missile_collision(missile: &mut Missile, spiders: &mut Spiders,
    base_bricks: &mut BaseBricks, letter_bricks: &mut LetterBricks,
    score: &mut u32) {

    if missile.flying() {
        if let Some(spider_id) = spiders.collision(missile.area()) {
            let target_brick_id = spiders.target_brick_id(spider_id);
            if let Some(brick_id) = target_brick_id {
                if spiders.carrying(spider_id) {
                    letter_bricks.untarget(brick_id);
                }
                else {
                    base_bricks.untarget(brick_id);
                }
            }
            missile.terminate_flight();
            let points = SPIDER_SCORE[spiders.spider_type(spider_id)];
            *score += points;
            if spiders.carrying(spider_id) {
                *score += points;
            }
            spiders.kill(spider_id);
        }
    }
}

pub fn bomb_collision(bombs: &mut Bombs, ship: &mut Ship) {
    if ship.alive() {
        if bombs.collision(ship.area()) {
            ship.kill();
        }
    }
}

pub fn spider_collision(spiders: &mut Spiders, ship: &mut Ship,
    base_bricks: &mut BaseBricks, letter_bricks: &mut LetterBricks) {
    if ship.alive() {
        if let Some(spider_id) = spiders.collision(ship.area()) {
            let target_brick_id = spiders.target_brick_id(spider_id);
            if let Some(brick_id) = target_brick_id {
                if spiders.carrying(spider_id) {
                    letter_bricks.untarget(brick_id);
                }
                else {
                    base_bricks.untarget(brick_id);
                }
            }
            spiders.kill(spider_id);
            ship.kill();
        }
    }
}
