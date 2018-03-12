use piston_window;
use piston_window::*;

use common;
use ship::Ship;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use missile::Missile;
use mother::Mother;
use spiders::Spiders;
use bombs::Bombs;
use soundfx::SoundFx;
use bonus_bomb::BonusBomb;
use animation::Animations;
use background::Background;

const SPIDER_SCORE: [u32; 3] = [40, 80, 200];
const EXTRA_LIFE_SCORE: u32 = 6000;

enum State {
    Startup,
    InProgress,
    GameOver,
}

impl State {
    fn playing(&self) -> bool {
        match *self {
            State::Startup => {false},
            State::GameOver => {false},
            _ => {true},
        }
    }

    fn screen_in_progress(&self) -> bool {
        match *self {
            State::InProgress => {true},
            _ => {false},
        }
    }
}

struct GameInput {
    left_pressed: bool,
    right_pressed: bool,
    fire_pressed: bool,
    start_pressed: bool,
    pause_pressed: bool,
    inc_vol_pressed: bool,
    dec_vol_pressed: bool,
}

impl GameInput {
    fn new() -> GameInput {
        GameInput {
            left_pressed: false,
            right_pressed: false,
            fire_pressed: false,
            start_pressed: false,
            pause_pressed: false,
            inc_vol_pressed: false,
            dec_vol_pressed: false,
        }
    }

    fn reset(&mut self) {
        self.left_pressed = false;
        self.right_pressed = false;
        self.fire_pressed = false;
        self.start_pressed = false;
        self.pause_pressed = false;
        self.inc_vol_pressed = false;
        self.dec_vol_pressed = false;
    }

    fn update_inputs(&mut self, e: &piston_window::Event) {
        match e.press_args() {
            Some(Button::Keyboard(Key::Z)) => {self.left_pressed = true;},
            Some(Button::Keyboard(Key::X)) => {self.right_pressed = true;},
            Some(Button::Keyboard(Key::RShift)) => {self.fire_pressed = true;},
            Some(Button::Keyboard(Key::Space)) => {self.start_pressed = true;},
            Some(Button::Keyboard(Key::P)) => {self.pause_pressed = true;},
            Some(Button::Keyboard(Key::Up)) => {self.inc_vol_pressed = true;},
            Some(Button::Keyboard(Key::Down)) => {self.dec_vol_pressed = true;},
            _ => {}
        }
        match e.release_args() {
            Some(Button::Keyboard(Key::Z)) => {self.left_pressed = false;},
            Some(Button::Keyboard(Key::X)) => {self.right_pressed = false;},
            Some(Button::Keyboard(Key::Space)) => {self.start_pressed = false;},
            _ => {}
        }
    }

    fn acknowledge_fire(&mut self) {
        self.fire_pressed = false;
    }

    fn acknowledge_pause(&mut self) {
        self.pause_pressed = false;
    }

    fn acknowledge_volume_change(&mut self) {
        self.inc_vol_pressed = false;
        self.dec_vol_pressed = false;
    }
}

pub struct Game {
    game_state: State,
    ship: Ship,
    missile: Missile,
    base_bricks: BaseBricks,
    letter_bricks: LetterBricks,
    mother: Mother,
    spiders: Spiders,
    bombs: Bombs,
    bonus_bomb: BonusBomb,
    game_over_image: G2dTexture,
    instructions_image: G2dTexture,
    screen_flag_image: G2dTexture,
    game_input: GameInput,
    frame_count: u32,
    score: u32,
    screen: u32,
    sound: SoundFx,
    paused: bool,
    animations: Animations,
    background: Background,
}

impl Game {
    pub fn new(window: &mut PistonWindow) -> Game {
        Game {
            game_state: State::Startup,
            ship: Ship::new(window),
            missile: Missile::new(window),
            base_bricks: BaseBricks::new(window),
            letter_bricks: LetterBricks::new(window),
            mother: Mother::new(window),
            spiders: Spiders::new(window),
            bombs: Bombs::new(window),
            bonus_bomb: BonusBomb::new(window),
            game_over_image: common::win_image(window, "game_over.png"),
            instructions_image: common::win_image(window, "instructions.png"),
            screen_flag_image: common::win_image(window, "screen_flag.png"),
            game_input: GameInput::new(),
            frame_count: 0,
            score: 0,
            screen: 0,
            sound: SoundFx::new(),
            paused: false,
            animations: Animations::new(),
            background: Background::new(window),
        }
    }

    fn new_game(&mut self) {
        self.game_state = State::InProgress;
        self.mother.full_reset();
        self.spiders.reset();
        self.ship.reset();
        self.missile.reset();
        self.base_bricks.reset();
        self.base_bricks.update();
        self.letter_bricks.reset();
        self.bombs.reset();
        self.bonus_bomb.reset();
        self.score = 0;
        self.frame_count = 0;
        self.screen = 1;
        self.game_input.reset();
        self.sound.turn_on();
        self.ship.proceed_with_changeover();
        self.screen_start();
    }

    fn increase_score(&mut self, inc: u32) {
        let q = self.score / EXTRA_LIFE_SCORE;
        self.score += inc;
        if (self.score / EXTRA_LIFE_SCORE) > q {
            self.ship.award_extra_life(&self.sound, &mut self.animations);
        }
    }

    fn render_score(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        text::Text::new_color([0.0, 0.0, 1.0, 1.0], 32).draw(
            &format!("{:07}", self.score),
            glyphs,
            &c.draw_state,
            c.transform.trans(common::SCREEN_WIDTH / 2.0 - 60.0, 32.0),
            g
            ).unwrap();
    }

    fn screen_start(&mut self) {
        let screen_number = self.screen;
        self.animations.register_text(
            Box::new(move |frame, c, g, gl| {
                text::Text::new_color([0.31, 0.47, 0.71, 1.0 - (frame as f32 / 100.0)], 40).draw(
                    &format!("Get ready for attack {}", screen_number),
                    gl,
                    &c.draw_state,
                    c.transform.trans(120.0, 350.0),
                    g
                    ).unwrap();
            }),
            100);
    }

    fn render_screens_complete(&self, c: Context, g: &mut G2d) {
        for i in 1..self.screen {
            image(
                &self.screen_flag_image,
                c.transform.trans(i as f64 * 20.0 - 15.0, common::SCREEN_HEIGHT - 22.0),
                g);
        }
    }

    // a collision means occupying the same space in the same frame
    // that may turn out to be too naive but it will do for now
    fn missile_collision(&mut self) {
        if self.missile.flying() {
            if let Some(spider_id) = self.spiders.collision(self.missile.area()) {
                let target_brick_id = self.spiders.target_brick_id(spider_id);
                if let Some(brick_id) = target_brick_id {
                    if self.spiders.carrying(spider_id) {
                        self.letter_bricks.untarget(brick_id);
                    }
                    else {
                        self.base_bricks.untarget(brick_id);
                    }
                }
                self.missile.terminate_flight();
                let points = SPIDER_SCORE[self.spiders.spider_type(spider_id)];
                self.increase_score(points);
                if self.spiders.carrying(spider_id) {
                    self.increase_score(points);
                }
                self.spiders.kill(spider_id, &self.sound, &mut self.animations);
            }
        }
    }

    fn bomb_collision(&mut self) {
        if self.ship.alive() {
            if self.bombs.collision(self.ship.area()) {
                self.ship.kill(&self.sound, &mut self.animations);
            }
        }
    }

    fn spider_collision(&mut self) {
        if self.ship.alive() {
            if let Some(spider_id) = self.spiders.collision(self.ship.area()) {
                let target_brick_id = self.spiders.target_brick_id(spider_id);
                if let Some(brick_id) = target_brick_id {
                    if self.spiders.carrying(spider_id) {
                        self.letter_bricks.untarget(brick_id);
                    }
                    else {
                        self.base_bricks.untarget(brick_id);
                    }
                }
                self.spiders.kill(spider_id, &self.sound, &mut self.animations);
                self.ship.kill(&self.sound, &mut self.animations);
            }
        }
    }

    fn bonus_bomb_collision(&mut self) {
        if self.missile.flying() && self.bonus_bomb.collision(self.missile.area()) {
            self.missile.terminate_flight();
            self.bonus_bomb.achieve_bonus(&mut self.letter_bricks, &mut self.animations);
            self.sound.bonus_bomb_hit();
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        if self.paused {
            if self.animations.in_progress() {
                // just so we can see the volume control when the game is paused
                // animations won't actually proceed without animations.update()
                self.animations.render(c, g, glyphs);
            }
            text::Text::new_color([0.0, 0.0, 1.0, 1.0], 32).draw(
                "Paused",
                glyphs,
                &c.draw_state,
                c.transform.trans(common::SCREEN_WIDTH / 2.0 - 55.0, 300.0),
                g
                ).unwrap();
        }
        else {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.background.render(c, g);
            self.render_screens_complete(c, g);
            self.base_bricks.render(c, g);
            self.letter_bricks.render(c, g);
            self.mother.render(c, g, self.frame_count);
            self.spiders.render(&self.mother, c, g, self.frame_count);
            self.bonus_bomb.render(c, g, self.frame_count);
            if self.game_state.playing() {
                self.ship.render(c, g, self.frame_count);
                self.missile.render(c, g);
            }
            self.bombs.render(c, g);
            self.render_score(c, g, glyphs);
            if let State::GameOver = self.game_state {
                image(&self.game_over_image, c.transform.trans(87.0, 250.0), g);
            }
            if ! self.game_state.playing() {
                image(&self.instructions_image, c.transform.trans(152.0, 350.0), g);
            }
            self.animations.render(c, g, glyphs);
        }
    }

    pub fn update_inputs(&mut self, e: &piston_window::Event) {
        self.game_input.update_inputs(e);
    }

    pub fn update(&mut self) {
        if self.game_input.pause_pressed {
            self.game_input.acknowledge_pause();
            if self.game_state.playing() {
                self.paused = ! self.paused;
            }
        }

        if self.game_input.dec_vol_pressed {
            self.game_input.acknowledge_volume_change();
            self.sound.decrease_volume(&mut self.animations);
        }
        if self.game_input.inc_vol_pressed {
            self.game_input.acknowledge_volume_change();
            self.sound.increase_volume(&mut self.animations);
        }

        if ! self.paused {
            self.frame_count += 1;

            if self.game_state.playing() {
                self.bonus_bomb_collision();
                self.missile_collision();
                self.bomb_collision();
                self.spider_collision();
                if self.ship.waiting_for_changeover() && self.spiders.clear() &&
                    ! self.bombs.in_flight() && ! self.missile.flying() {
                    self.ship.proceed_with_changeover();
                }
                if self.letter_bricks.complete() || ! self.ship.life_left() {
                    self.game_state = State::GameOver;
                    self.sound.turn_off();
                }

                self.missile.update();
                self.ship.update();
                if self.game_input.left_pressed {
                    self.ship.move_left();
                }
                else if self.game_input.right_pressed {
                    self.ship.move_right();
                }
                if self.game_input.fire_pressed {
                    self.ship.launch_missile(&mut self.missile, &self.sound);
                    self.game_input.acknowledge_fire();
                }
            }

            if self.game_state.screen_in_progress() || ! self.game_state.playing() {
                self.base_bricks.update();
                self.letter_bricks.update(&self.sound, self.frame_count);
                self.bonus_bomb.update(&self.sound);
                self.mother.update(&mut self.bonus_bomb, self.frame_count);
                self.bombs.update();
                self.spiders.update(
                    &self.mother,
                    &mut self.base_bricks,
                    &mut self.letter_bricks,
                    &mut self.bombs,
                    self.ship.in_changeover() && self.game_state.playing(),
                    self.frame_count,
                    &self.sound);
            }

            if self.game_state.screen_in_progress() &&
                ! self.spiders.spiders_remain() &&
                ! self.bombs.in_flight() {
                self.screen += 1;
                self.mother.reset();
                self.bonus_bomb.reset();
                self.spiders.reset();
                self.frame_count = 0;
                self.screen_start();
            }

            if (! self.game_state.playing()) && self.game_input.start_pressed {
                self.new_game();
            }
            self.animations.update();
            self.background.update();
        }
    }
}

