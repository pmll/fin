use std::path::PathBuf;
use music;

use common;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Fire,
    ShipExplode,
    SpiderExplode,
    TakeBrick,
    DepositBrick,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {}

pub struct SoundFx {
    on: bool,
}

impl SoundFx {
    pub fn new() -> SoundFx {
        SoundFx {on: false}
    }

    fn play_sound(&self, sound: Sound) {
        if self.on {
            music::play_sound(&sound, music::Repeat::Times(0), music::MAX_VOLUME);
        }
    }

    pub fn bind_sound_files() {
        music::bind_sound_file(Sound::Fire, find_sound_asset("fire.wav"));
        music::bind_sound_file(Sound::TakeBrick, find_sound_asset("grab.wav"));
        music::bind_sound_file(Sound::DepositBrick, find_sound_asset("drop.wav"));
        music::bind_sound_file(Sound::SpiderExplode, find_sound_asset("spider_explosion.wav"));
        music::bind_sound_file(Sound::ShipExplode, find_sound_asset("ship_explosion.wav"));
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

    pub fn spider_explode(&self) {
        self.play_sound(Sound::SpiderExplode);
    }

    pub fn ship_explode(&self) {
        self.play_sound(Sound::ShipExplode);
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
    }
}

fn find_sound_asset(file_name: &str) -> PathBuf {
    common::find_asset(&format!("sound/{}", file_name))
}
