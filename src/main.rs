// FIN
extern crate piston_window;
extern crate music;
extern crate find_folder;
extern crate rand;

mod common;
mod ship;
mod base_bricks;
mod letter_bricks;
mod missile;
mod mother;
mod spiders;
mod bombs;
mod collision;

use piston_window::*;

use ship::Ship;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use missile::Missile;
use mother::Mother;
use spiders::Spiders;
use bombs::Bombs;

enum GameState {
    Startup,
    ScreenStart(f32),
    InProgress,
    GameOver,
}

impl GameState {
    fn playing(&self) -> bool {
        match *self {
            GameState::Startup => {false},
            GameState::GameOver => {false},
            _ => {true},
        }
    }

    fn screen_in_progress(&self) -> bool {
        match *self {
            GameState::InProgress => {true},
            _ => {false},
        }
    }
}

fn main() {
    let opengl = OpenGL::V2_1;
    let mut window: PistonWindow = WindowSettings::new(
        "FIN",
        (common::SCREEN_WIDTH as u32, common::SCREEN_HEIGHT as u32)
    )
    .exit_on_esc(true)
    .opengl(opengl)
    .build()
    .unwrap();

    window.set_ups(60);

    let mut ship = Ship::new(&mut window);
    let mut missile = Missile::new(&mut window);
    let mut base_bricks = BaseBricks::new(&mut window);
    let mut letter_bricks = LetterBricks::new(&mut window);
    let mut mother = Mother::new(&mut window);
    let mut spiders = Spiders::new(&mut window);
    let mut bombs = Bombs::new(&mut window);
 
    let ref font = common::find_asset("font/FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut fire_pressed = false;
    let mut start_pressed = false;
    let mut frame_count = 0;
    let mut game_state = GameState::Startup;
    let mut score = 0;
    let mut screen = 0;

    music::start::<common::Music, common::Sound, _>(16, || {
        music::bind_sound_file(common::Sound::Fire, common::find_asset("sound/fire.wav"));
        music::bind_sound_file(common::Sound::TakeBrick, common::find_asset("sound/grab.wav"));
        music::bind_sound_file(common::Sound::DepositBrick, common::find_asset("sound/drop.wav"));
        music::bind_sound_file(common::Sound::SpiderExplode, common::find_asset("sound/spider_explosion.wav"));
        music::bind_sound_file(common::Sound::ShipExplode, common::find_asset("sound/ship_explosion.wav"));

        while let Some(e) = window.next() {
            if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    base_bricks.render(c, g);
                    letter_bricks.render(c, g);
                    mother.render(c, g, frame_count);
                    spiders.render(&mother, c, g, frame_count);
                    if game_state.playing() {
                        ship.render(c, g, frame_count);
                        missile.render(c, g);
                    }
                    bombs.render(c, g);
                    text::Text::new_color([0.0, 0.0, 1.0, 1.0], 32).draw(
                        &format!("{:07}", score),
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(common::SCREEN_WIDTH / 2.0 - 60.0, 32.0),
                        g
                        ).unwrap();
                    if let GameState::GameOver = game_state {
                        render_text(c, g, &mut glyphs, 100.0, 300.0, 64, "Game Over");
                    }
                    if ! game_state.playing() {
                        render_text(c, g, &mut glyphs, 100.0, 350.0, 32, "Play Fin");
                        render_text(c, g, &mut glyphs, 100.0, 400.0, 20, "Don't let the spiders spell their word");
                        render_text(c, g, &mut glyphs, 100.0, 450.0, 32, "Press space to play");
                        render_text(c, g, &mut glyphs, 100.0, 500.0, 32, "Press escape to exit at any time");
                    }
                    if let GameState::ScreenStart(fade) = game_state {
                        if fade > 0.0 {
                            render_attack_text(c, g, &mut glyphs, screen, fade);
                            game_state = GameState::ScreenStart(fade - 0.01);
                        }
                        else {
                            game_state = GameState::InProgress;
                        }
                    }
                });
            }

            match e.press_args() {
                Some(Button::Keyboard(Key::Z)) => {left_pressed = true;},
                Some(Button::Keyboard(Key::X)) => {right_pressed = true;},
                Some(Button::Keyboard(Key::RShift)) => {fire_pressed = true;},
                Some(Button::Keyboard(Key::Space)) => {start_pressed = true;}
                _ => {}
            }

            match e.release_args() {
                Some(Button::Keyboard(Key::Z)) => {left_pressed = false;},
                Some(Button::Keyboard(Key::X)) => {right_pressed = false;},
                //Some(Button::Keyboard(Key::RShift)) => {fire_pressed = false;},
                Some(Button::Keyboard(Key::Space)) => {start_pressed = false;}
                _ => {}
            }

            if let Some(_) = e.update_args() {
                frame_count += 1;
                
                if game_state.playing() {
                    if left_pressed {
                        ship.move_left();
                    }
                    else if right_pressed {
                        ship.move_right();
                    }
                    if fire_pressed {
                        ship.launch_missile(&mut missile);
                        fire_pressed = false;
                    }
                    missile.update();
                    ship.update();
                }
                if game_state.screen_in_progress() || ! game_state.playing() {
                    base_bricks.update();
                    mother.update();
                    bombs.update();
                    spiders.update(&mother, &mut base_bricks, &mut letter_bricks,
                        &mut bombs, ship.in_changeover() && game_state.playing(), frame_count);
                }
                if game_state.playing() {
                    collision::missile_collision(&mut missile, &mut spiders,
                        &mut base_bricks, &mut letter_bricks, &mut score);
                    collision::bomb_collision(&mut bombs, &mut ship);
                    collision::spider_collision(&mut spiders, &mut ship,
                        &mut base_bricks, &mut letter_bricks);
                    if ship.waiting_for_changeover() && spiders.clear() && ! bombs.in_flight() {
                        ship.proceed_with_changeover();
                    }
                    if letter_bricks.complete() || ! ship.life_left() {
                        game_state = GameState::GameOver;
                        common::sound_off();
                    }
                }
                if game_state.screen_in_progress() && ! spiders.spiders_remain() && ! bombs.in_flight() {
                    screen += 1;
                    game_state = GameState::ScreenStart(1.0);
                    mother.reset();
                    spiders.reset();
                    frame_count = 0;
                }
                if (! game_state.playing()) && start_pressed {
                    game_state = GameState::ScreenStart(1.0);
                    mother.reset();
                    spiders.reset();
                    ship.reset();
                    missile.reset();
                    base_bricks.reset();
                    base_bricks.update();
                    letter_bricks.reset();
                    bombs.reset();
                    score = 0;
                    frame_count = 0;
                    screen = 1;
                    fire_pressed = false;
                    common::sound_on();
                    ship.proceed_with_changeover();
                }
            }
        }
    });
}


fn render_text(c: Context, g: &mut G2d, glyphs: &mut Glyphs, x: f64, y: f64, sz: u32, txt: &str) {
    text::Text::new_color([1.0, 1.0, 1.0, 1.0], sz).draw(
        txt,
        glyphs,
        &c.draw_state,
        c.transform.trans(x, y),
        g
        ).unwrap();
}

fn render_attack_text(c: Context, g: &mut G2d, glyphs: &mut Glyphs, screen: u32, fade: f32) {
    text::Text::new_color([0.31, 0.47, 0.71, fade], 40).draw(
        &format!("Get ready for attack {}", screen),
        glyphs,
        &c.draw_state,
        c.transform.trans(120.0, 350.0),
        g
        ).unwrap();
}
