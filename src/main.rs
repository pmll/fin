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
mod soundfx;
mod game;
mod bonus_bomb;
mod animation;
mod background;

use piston_window::*;

use game::Game;
use soundfx::SoundFx;

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

    let ref font = common::find_asset("font/FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut game = Game::new(&mut window);

    music::start::<soundfx::Music, soundfx::Sound, _>(16, || {
        SoundFx::bind_sound_files();

        while let Some(e) = window.next() {
            if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                    game.render(c, g, &mut glyphs);
                });
            }

            game.update_inputs(&e);

            if let Some(_) = e.update_args() {
                game.update();
            }
        }
    });
}

