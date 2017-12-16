use common;
use spiders::Spiders;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use missile::Missile;

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
            spiders.kill(spider_id);
            // todo: score
        }
    }
}
