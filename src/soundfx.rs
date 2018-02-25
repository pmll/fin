use std::path::PathBuf;
use music;
use piston_window::*;
use piston_window::rectangle;
use animation::Animations;

use common;

const VOL_DISP_X: f64 = 245.0;
const VOL_DISP_Y: f64 = 240.0;
const VOL_BAR_DISP_WIDTH: f64 = 10.0;
const MAX_VOL_DISP_HEIGHT: f64 = 30.0;
const VOL_STEPS: u32 = 10;
const VOL_STEP: f64 = 1.0 / (VOL_STEPS as f64);

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Fire,
    ShipExplode,
    SpiderExplode,
    TakeBrick,
    DepositBrick,
    BonusBomb,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {}

pub struct SoundFx {
    on: bool,
    volume: f64,
}

impl SoundFx {
    pub fn new() -> SoundFx {
        SoundFx {on: false, volume: music::MAX_VOLUME}
    }

    fn play_sound(&self, sound: Sound) {
        if self.on {
            music::play_sound(&sound, music::Repeat::Times(0), self.volume);
        }
    }

    fn volume_change(&self, animations: &mut Animations) {
        let vol = self.volume;
        animations.register(
            Box::new(move |frame, c, g| {
                let bar_colour = [0.0, 0.0, 1.0, 1.0 - (frame as f32) / 100.0];
                let blank_bar_colour = [0.0, 0.0, 0.0, 1.0];
                let vol_steps = VOL_STEPS as f64;

                rectangle(bar_colour,
                          [VOL_DISP_X, VOL_DISP_Y, VOL_BAR_DISP_WIDTH * (vol_steps + 1.0), 2.0],
                          c.transform.trans(0.0, 0.0), g);

                for i in 0..VOL_STEPS + 1 {
                    let bar_n = (i as f64) / vol_steps;
                    let bar_x = VOL_DISP_X + (i as f64) * VOL_BAR_DISP_WIDTH;
                    let bar_h = MAX_VOL_DISP_HEIGHT * (i as f64) / vol_steps;
                    let bar = [bar_x, VOL_DISP_Y - 2.0, VOL_BAR_DISP_WIDTH - 1.0, - bar_h];
                    rectangle(if bar_n > vol {blank_bar_colour} else {bar_colour},
                              bar, c.transform.trans(0.0, 0.0), g);

                }
            }),
            100);
    }
    pub fn bind_sound_files() {
        music::bind_sound_file(Sound::Fire, find_sound_asset("fire.wav"));
        music::bind_sound_file(Sound::TakeBrick, find_sound_asset("grab.wav"));
        music::bind_sound_file(Sound::DepositBrick, find_sound_asset("drop.wav"));
        music::bind_sound_file(Sound::SpiderExplode, find_sound_asset("spider_explosion.wav"));
        music::bind_sound_file(Sound::ShipExplode, find_sound_asset("ship_explosion.wav"));
        music::bind_sound_file(Sound::BonusBomb, find_sound_asset("bonus_bomb.wav"));
    }

    pub fn fire(&self) {
        self.play_sound(Sound::Fire);
    }

    pub fn take_brick(&self) {
        self.play_sound(Sound::TakeBrick);
    }

    pub fn deposit_brick(&self) {
        self.play_sound(Sound::DepositBrick);
    }

    pub fn remove_brick(&self) {
        self.play_sound(Sound::TakeBrick); // for now
    }

    pub fn spider_explode(&self) {
        self.play_sound(Sound::SpiderExplode);
    }

    pub fn ship_explode(&self) {
        self.play_sound(Sound::ShipExplode);
    }

    pub fn bonus_bomb_hit(&self) {
        self.play_sound(Sound::SpiderExplode);  // for now
    }

    pub fn bonus_bomb(&self) {
        self.play_sound(Sound::BonusBomb);
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
    }

    pub fn increase_volume(&mut self, animations: &mut Animations) {
        self.volume += VOL_STEP;
        self.volume = self.volume.min(music::MAX_VOLUME);
        self.volume_change(animations);
    }

    pub fn decrease_volume(&mut self, animations: &mut Animations) {
        self.volume -= VOL_STEP;
        self.volume = self.volume.max(music::MIN_VOLUME);
        self.volume_change(animations);
    }
}

fn find_sound_asset(file_name: &str) -> PathBuf {
    common::find_asset(&format!("sound/{}", file_name))
}
