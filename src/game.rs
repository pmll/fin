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

const SPIDER_SCORE: [u32; 3] = [40, 80, 200];

enum State {
    Startup,
    ScreenStart(f32),
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
}

impl GameInput {
    fn new() -> GameInput {
        GameInput {
            left_pressed: false,
            right_pressed: false,
            fire_pressed: false,
            start_pressed: false,
        }
    }

    fn reset(&mut self) {
        self.left_pressed = false;
        self.right_pressed = false;
        self.fire_pressed = false;
        self.start_pressed = false;
    }

    fn update_inputs(&mut self, e: &piston_window::Event) {
        match e.press_args() {
            Some(Button::Keyboard(Key::Z)) => {self.left_pressed = true;},
            Some(Button::Keyboard(Key::X)) => {self.right_pressed = true;},
            Some(Button::Keyboard(Key::RShift)) => {self.fire_pressed = true;},
            Some(Button::Keyboard(Key::Space)) => {self.start_pressed = true;}
            _ => {}
        }
        match e.release_args() {
            Some(Button::Keyboard(Key::Z)) => {self.left_pressed = false;},
            Some(Button::Keyboard(Key::X)) => {self.right_pressed = false;},
            Some(Button::Keyboard(Key::Space)) => {self.start_pressed = false;}
            _ => {}
        }
    }

    fn acknowledge_fire(&mut self) {
        self.fire_pressed = false;
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
    game_over_image: G2dTexture,
    instructions_image: G2dTexture,
    game_input: GameInput,
    frame_count: u32,
    score: u32,
    screen: u32,
    sound: SoundFx,
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
            game_over_image: common::win_image(window, "game_over.png"),
            instructions_image: common::win_image(window, "instructions.png"),
            game_input: GameInput::new(),
            frame_count: 0,
            score: 0,
            screen: 0,
            sound: SoundFx::new(),
        }
    }

    fn new_game(&mut self) {
        self.game_state = State::ScreenStart(1.0);
        self.mother.reset();
        self.spiders.reset();
        self.ship.reset();
        self.missile.reset();
        self.base_bricks.reset();
        self.base_bricks.update();
        self.letter_bricks.reset();
        self.bombs.reset();
        self.score = 0;
        self.frame_count = 0;
        self.screen = 1;
        self.game_input.reset();
        self.sound.turn_on();
        self.ship.proceed_with_changeover();
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

    fn render_attack_text(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs, fade: f32) {
        text::Text::new_color([0.31, 0.47, 0.71, fade], 40).draw(
            &format!("Get ready for attack {}", self.screen),
            glyphs,
            &c.draw_state,
            c.transform.trans(120.0, 350.0),
            g
            ).unwrap();
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
                self.score += points;
                if self.spiders.carrying(spider_id) {
                    self.score += points;
                }
                self.spiders.kill(spider_id, &self.sound);
            }
        }
    }

    fn bomb_collision(&mut self) {
        if self.ship.alive() {
            if self.bombs.collision(self.ship.area()) {
                self.ship.kill(&self.sound);
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
                self.spiders.kill(spider_id, &self.sound);
                self.ship.kill(&self.sound);
            }
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.base_bricks.render(c, g);
        self.letter_bricks.render(c, g);
        self.mother.render(c, g, self.frame_count);
        self.spiders.render(&self.mother, c, g, self.frame_count);
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
        if let State::ScreenStart(n) = self.game_state {
            self.render_attack_text(c, g, glyphs, n);
        }
    }

    pub fn update_inputs(&mut self, e: &piston_window::Event) {
        self.game_input.update_inputs(e);
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.game_state.playing() {
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
            self.mother.update();
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

        if self.game_state.playing() {
            self.missile_collision();
            self.bomb_collision();
            self.spider_collision();
            if self.ship.waiting_for_changeover() && self.spiders.clear() &&
                ! self.bombs.in_flight() {
                self.ship.proceed_with_changeover();
            }
            if self.letter_bricks.complete() || ! self.ship.life_left() {
                self.game_state = State::GameOver;
                self.sound.turn_off();
            }
        }

        if self.game_state.screen_in_progress() &&
            ! self.spiders.spiders_remain() &&
            ! self.bombs.in_flight() {
            self.screen += 1;
            self.game_state = State::ScreenStart(1.0);
            self.mother.reset();
            self.spiders.reset();
            self.frame_count = 0;
        }

        if (! self.game_state.playing()) && self.game_input.start_pressed {
            self.new_game();
        }

        if let State::ScreenStart(n) = self.game_state {
            if n > 0.0 {
                self.game_state = State::ScreenStart(n - 0.01);
            }
            else {
                self.game_state = State::InProgress;
            }
        }
    }
}

